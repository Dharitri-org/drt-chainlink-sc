package aggregator

import (
	"testing"

	"github.com/DharitriNetwork/dharitri-adapter/config"
	"github.com/stretchr/testify/require"
)

const (
	okBaseTicker  = "ETH"
	USDQuote      = "USD"
	errBaseTicker = "ETHZ"
	moaxTicker    = "MOAX"
)

func TestExchangeAggregator_GetPriceMultipliedShouldWork(t *testing.T) {
	t.Parallel()
	aggregator := NewExchangeAggregator(config.ExchangeConfig{})
	price, err := aggregator.GetPrice(okBaseTicker, USDQuote)
	require.Nil(t, err)
	require.True(t, price > 0)
}

func TestExchangeAggregator_GetPriceMultipliedShouldErr(t *testing.T) {
	t.Parallel()
	aggregator := NewExchangeAggregator(config.ExchangeConfig{})
	price, err := aggregator.GetPrice(errBaseTicker, USDQuote)
	require.Error(t, err)
	require.True(t, price == -1)
}

func TestExchangeAggregator_GetPriceMultipliedMOAX(t *testing.T) {
	t.Parallel()
	aggregator := NewExchangeAggregator(config.ExchangeConfig{})
	price, err := aggregator.GetPrice(moaxTicker, USDQuote)
	require.Nil(t, err)
	require.True(t, price > 0)
}
