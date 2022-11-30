package main

// Today's code isn't great, I was hoping to use quadratic equations, but decided to get something working
// quickly instead

import (
	"fmt"
	"log"
)

type Projectile struct {
	X         int
	Y         int
	XVelocity int
	YVelocity int
	Target
}

type Target struct {
	XMin int
	XMax int
	YMin int
	YMax int
}

func (p *Projectile) Step() bool {
	p.X += p.XVelocity
	p.Y += p.YVelocity

	if p.XVelocity < 0 {
		p.XVelocity += 1
	} else if p.XVelocity > 0 {
		p.XVelocity -= 1
	}

	p.YVelocity -= 1

	// If the projectile is in the target
	if p.X >= p.Target.XMin && p.X <= p.Target.XMax && p.Y >= p.Target.YMin && p.Y <= p.Target.YMax {
		return true
	}

	return false
}

// PassedTarget assumes the target is on a lower Y and greater X coordinate than 0,0
func (p *Projectile) PassedTarget() bool {
	return p.X > p.Target.XMax || p.Y < p.Target.YMin
}

func main() {
	// example
	//t := Target{
	//	XMin: 20,
	//	XMax: 30,
	//	YMin: -10,
	//	YMax: -5,
	//}

	// actual
	t := Target{
		XMin: 85,
		XMax: 145,
		YMin: -163,
		YMax: -108,
	}

	highest := 0
	count := 0

	// these values are quite big just to make sure the right answer is got
	for xV := -400; xV < 400; xV++ {
		for yV := -400; yV < 400; yV++ {
			// create a new projectile with initial position 0,0 and xV,yV velocity
			p := Projectile{
				X:         0,
				Y:         0,
				XVelocity: xV,
				YVelocity: yV,
				Target:    t,
			}

			// maximum y value on this path
			maxY := 0
			inTarget := false

			// while true
			for {
				inTarget = p.Step()

				if p.Y > maxY {
					maxY = p.Y
				}

				// if the target has been hit, or it has gone past the target
				if inTarget || p.PassedTarget() {
					break
				}
			}

			// don't care about this projectile if it didn't hit
			if !inTarget {
				continue
			}

			if maxY > highest {
				highest = maxY
			}

			log.Printf("valid velocity %v,%v - height %v", xV, yV, maxY)
			count += 1
		}
	}

	fmt.Printf("part 1: %v\n", highest)
	fmt.Printf("part 2: %v\n", count)
}
