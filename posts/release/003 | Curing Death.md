# Curing Death

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*tDIy2jhVd8z2HTjt7CY_rA.jpeg)

**Approximating biology in silicon will allow us to understand senescence, error-correct cellular information loss, and cure death.**

Recently I read the great article [can a biologist fix a radio?](https://www.cell.com/cancer-cell/pdf/S1535-6108(02)00133-2.pdf) I recommend anyone interested in biology but not currently working in it give it a read. It really helps someone like me, used to the clean perfection of abstracted bit processing, to grasp just how difficult a problem the entire medical industry is dealing with, and the flaws in the current reductionist / empiricist approach. It's the hardest debugging problem of all time.

By far the worst part of this problem is just how difficult it is to run tests. When I face a problem with my code, I can run it 50 different ways in a few minutes, testing out different inputs to see how it behaves. Now imagine how long it would take to find a bug if you could only run your code a few times over 3 years, and each time you hit run it costs $41,000. Quite the challenge!

And yet that's the exact situation researchers are contending with. The costs of clinical trials [have spiraled out of control](https://www.bls.gov/opub/mlr/2014/article/price-indexes-for-clinical-trial-research-a-feasibility-study.htm), and FDA approval is harder than ever to recieve. So how does this get solved? The obvious answer is political. The FDA should be slimmed down, the approval process made more transparent, the discovery process subsidized. But I'm not interested in political solutions.

## Taking inspiration from aerospace

Believe it or not, the aerospace industry faced a very similar problem in the mid-20th century. As aircraft got bigger, wind tunnels needed to grow to fit models that would have similar behavior as the full scale version. As aircraft got faster, wind tunnels needed to get more powerful, eventually requireing supersonic wind tunnels to model transsonic and supersonic flight charactaristics. These tunnels are incredibly expensive, and still can only operate on scale models.

Modern aircraft designs spend far less time in the tunnel, at a much smaller scale, because most design and testing happens ahead of time on comparatively cheap **computational fluid dynamics (CFD) simulations.** These sims are able to model air flow over a rigidbody, predict drag, lift, and even structural stresses on the airframe, all before any physical model needs to be constructed.

## First steps
