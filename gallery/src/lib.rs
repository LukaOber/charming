#![allow(clippy::type_complexity)]

use charming::Chart;
use std::{collections::BTreeMap, sync::LazyLock};

mod aria;
mod bar;
mod bar3d;
mod boxplot;
mod calendar;
mod candlestick;
mod dataset;
mod funnel;
mod gauge;
mod geo;
mod graph;
mod heatmap;
mod line;
mod parallel;
mod pictorial_bar;
mod pie;
mod radar;
mod sankey;
mod scatter;
mod sunburst;
mod theme_river;
mod tree;
mod treemap;

macro_rules! insert {
    ($map:ident, $type:ident, $name:ident) => {
        $map.insert(stringify!($name), $type::$name::chart as fn() -> Chart);
    };
}

static ARIA_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, aria, default_decal);
    m
});
static BAR_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, bar, bar_with_background);
    insert!(m, bar, basic_bar);
    insert!(m, bar, radial_polar_bar_label_position);
    insert!(m, bar, set_style_of_single_bar);
    insert!(m, bar, stacked_column);
    insert!(m, bar, tangential_polar_bar);
    insert!(m, bar, waterfall);
    insert!(m, bar, world_population);
    m
});
static BAR3D_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, bar3d, bar3d_with_dataset);
    m
});
static BOXPLOT_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, boxplot, basic_boxplot);
    insert!(m, boxplot, boxplot_light_velocity);
    insert!(m, boxplot, boxplot_light_velocity2);
    insert!(m, boxplot, data_transform_simple_aggregate);
    insert!(m, boxplot, multiple_categories);
    m
});
static CALENDAR_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, calendar, simple_calendar);
    insert!(m, calendar, heatmap_calendar);
    m
});
static CANDLESTICK_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, candlestick, basic_candlestick);
    insert!(m, candlestick, ohlc);
    insert!(m, candlestick, shanghai_index);
    m
});
static DATASET_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, dataset, encode_and_matrix);
    m
});
static FUNNEL_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, funnel, funnel_chart);
    insert!(m, funnel, multiple_funnels);
    m
});
static GAUGE_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, gauge, gauge_barometer);
    insert!(m, gauge, gauge_basic);
    insert!(m, gauge, gauge_simple);
    m
});
static GEO_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, geo, organ_data);
    m
});
static GRAPH_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, graph, hide_overlapped_label);
    insert!(m, graph, les_miserables);
    m
});
static HEATMAP_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, heatmap, heatmap_on_cartesian);
    m
});
static LINE_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, line, area_pieces);
    insert!(m, line, basic_area);
    insert!(m, line, basic_line);
    insert!(m, line, different_symbols);
    insert!(m, line, confidence_band);
    insert!(m, line, data_transform_filter);
    insert!(m, line, distribution_of_electricity);
    insert!(m, line, gradient_stacked_area);
    insert!(m, line, large_scale_area);
    insert!(m, line, line_gradient);
    insert!(m, line, rainfall);
    insert!(m, line, rainfall_vs_evaporation);
    insert!(m, line, smoothed_line);
    insert!(m, line, stacked_area);
    insert!(m, line, stacked_line);
    insert!(m, line, split_legend);
    insert!(m, line, step_line);
    insert!(m, line, temperature_change);
    insert!(m, line, time_axis);
    insert!(m, line, two_value_axes_in_polar);
    m
});
static PARALLEL_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, parallel, basic_parallel);
    insert!(m, parallel, parallel_aqi);
    insert!(m, parallel, parallel_nutrients);
    m
});
static PICTORIAL_BAR_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> =
    LazyLock::new(|| {
        let mut m = BTreeMap::new();
        insert!(m, pictorial_bar, water_content);
        m
    });
static PIE_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, pie, doughnut_chart_with_rounded_corner);
    insert!(m, pie, nightingale);
    insert!(m, pie, referer_of_a_website);
    m
});
static RADAR_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, radar, basic_radar);
    insert!(m, radar, multiple_radar);
    insert!(m, radar, proportion_of_browsers);
    m
});
static SANKEY_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, sankey, basic_sankey);
    insert!(m, sankey, node_align_left_sankey);
    insert!(m, sankey, sankey_orient_vertical);
    m
});
static SCATTER_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, scatter, anscombe_quartet);
    insert!(m, scatter, basic_scatter);
    insert!(m, scatter, bubble_chart);
    insert!(m, scatter, clustering_process);
    insert!(m, scatter, effect_scatter);
    insert!(m, scatter, punch_card_of_github);
    m
});
static SUNBURST_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, sunburst, drink_flavors);
    m
});
static THEME_RIVER_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, theme_river, theme_river_lastfm);
    m
});
static TREE_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, tree, from_left_to_right_tree);
    insert!(m, tree, multiple_trees);
    m
});
static TREEMAP_CHARTS: LazyLock<BTreeMap<&'static str, fn() -> Chart>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    insert!(m, treemap, disk_usage);
    m
});
pub static CHARTS: LazyLock<BTreeMap<&'static str, BTreeMap<&'static str, fn() -> Chart>>> =
    LazyLock::new(|| {
        let mut m = BTreeMap::new();
        m.insert("aria", ARIA_CHARTS.clone());
        m.insert("bar", BAR_CHARTS.clone());
        m.insert("bar3d", BAR3D_CHARTS.clone());
        m.insert("boxplot", BOXPLOT_CHARTS.clone());
        m.insert("calendar", CALENDAR_CHARTS.clone());
        m.insert("candlestick", CANDLESTICK_CHARTS.clone());
        m.insert("dataset", DATASET_CHARTS.clone());
        m.insert("funnel", FUNNEL_CHARTS.clone());
        m.insert("gauge", GAUGE_CHARTS.clone());
        m.insert("geo", GEO_CHARTS.clone());
        m.insert("graph", GRAPH_CHARTS.clone());
        m.insert("heatmap", HEATMAP_CHARTS.clone());
        m.insert("line", LINE_CHARTS.clone());
        m.insert("parallel", PARALLEL_CHARTS.clone());
        m.insert("pictorial_bar", PICTORIAL_BAR_CHARTS.clone());
        m.insert("pie", PIE_CHARTS.clone());
        m.insert("radar", RADAR_CHARTS.clone());
        m.insert("sankey", SANKEY_CHARTS.clone());
        m.insert("scatter", SCATTER_CHARTS.clone());
        m.insert("sunburst", SUNBURST_CHARTS.clone());
        m.insert("theme_river", THEME_RIVER_CHARTS.clone());
        m.insert("tree", TREE_CHARTS.clone());
        m.insert("treemap", TREEMAP_CHARTS.clone());
        m
    });
