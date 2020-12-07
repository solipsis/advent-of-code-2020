package main

import (
	"bufio"
	"fmt"
	"log"
	"math/big"
	"os"
	"strconv"
	"strings"
)

type node struct {
	name     string
	counts   map[string]*big.Int
	children map[string]int
}

func main() {
	var graph = make(map[string]*node)

	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("unable to open input: %v", err)
	}
	sc := bufio.NewScanner(f)

	for sc.Scan() {
		arr := strings.Split(sc.Text(), " ")
		name := concat(arr[:2])

		children := parseDeps(arr[4:])
		n := &node{
			name:     name,
			children: children,
		}
		graph[name] = n
	}
	// populate all counts
	for _, n := range graph {
		count(n, &graph)
	}

	// see how many have gold
	goldHolders := 0
	for k, n := range graph {
		fmt.Printf("%s: %+v\n", k, n)
		if n.counts["shiny-gold"].Cmp(big.NewInt(0)) > 0 {
			goldHolders++
		}
	}
	fmt.Println("Gold Holders:", goldHolders)

	// count bags that make up a gold bag
	var sum = big.NewInt(0)
	for _, v := range graph["shiny-gold"].counts {
		sum.Add(sum, v)
	}
	fmt.Println("Sum:", sum)
}

func count(n *node, g *map[string]*node) map[string]*big.Int {

	// this node has already calculated its children
	if n.counts != nil {
		return n.counts
	}

	counts := make(map[string]*big.Int)
	for childName, childNum := range n.children {

		childCounts := count((*g)[childName], g)
		counts[childName] = big.NewInt(int64(childNum))

		for k, v := range childCounts {
			if counts[k] == nil {
				counts[k] = big.NewInt(0)
			}
			// multiply by how many we have of this child
			counts[k].Add(counts[k], v.Mul(v, big.NewInt(int64(childNum))))
		}
	}
	n.counts = counts
	return counts
}

func parseDeps(arr []string) map[string]int {
	m := make(map[string]int)
	// bag contains no bags
	if len(arr) == 3 {
		return m
	}

	for len(arr) >= 4 {
		count, err := strconv.Atoi(arr[0])
		if err != nil {
			log.Fatalf("parsing dep: %v", err)
		}
		name := concat(arr[1:3])
		m[name] = count

		arr = arr[4:]
	}
	if len(arr) != 0 {
		log.Fatalf("I made an incorrect assumption about bag dependencies")
	}
	return m
}

func concat(arr []string) string {
	return strings.Join(arr, "-")
}
