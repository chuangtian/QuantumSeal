// Fixture: ordinary business logic with no cryptographic indicators.
// Used to verify quantumseal does not produce false positives on plain code.

package inventory

type Item struct {
	SKU      string
	Quantity int
	Price    float64
}

func TotalValue(items []Item) float64 {
	total := 0.0
	for _, it := range items {
		total += it.Price * float64(it.Quantity)
	}
	return total
}
