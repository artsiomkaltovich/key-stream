2026-08-02T22:16:31.3587981Z [1m[92m     Running[0m benches/key_stream.rs (target/release/deps/key_stream-b553559b340abd67)
2026-08-02T22:16:31.3600963Z Gnuplot not found, using plotters backend
2026-08-02T22:16:32.0378742Z Benchmarking full_cycle/1x1/1key_10000msg/u64
2026-08-02T22:16:32.0379993Z Benchmarking full_cycle/1x1/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:16:32.8680790Z Benchmarking full_cycle/1x1/1key_10000msg/u64: Collecting 20 samples in estimated 3.0216 s (1860 iterations)
2026-08-02T22:16:35.8880499Z Benchmarking full_cycle/1x1/1key_10000msg/u64: Analyzing
2026-08-02T22:16:35.9201028Z full_cycle/1x1/1key_10000msg/u64
2026-08-02T22:16:35.9201795Z                         time:   [1.6145 ms 1.6225 ms 1.6338 ms]
2026-08-02T22:16:35.9202441Z                         thrpt:  [30.604 Melem/s 30.818 Melem/s 30.970 Melem/s]
2026-08-02T22:16:35.9203117Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:16:35.9203646Z   3 (15.00%) low severe
2026-08-02T22:16:35.9204013Z   1 (5.00%) high severe
2026-08-02T22:16:35.9205237Z Benchmarking full_cycle/1x1/1key_10000msg/String
2026-08-02T22:16:35.9205968Z Benchmarking full_cycle/1x1/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:16:36.4951029Z Benchmarking full_cycle/1x1/1key_10000msg/String: Collecting 20 samples in estimated 3.0752 s (680 iterations)
2026-08-02T22:16:39.4445382Z Benchmarking full_cycle/1x1/1key_10000msg/String: Analyzing
2026-08-02T22:16:39.4741611Z full_cycle/1x1/1key_10000msg/String
2026-08-02T22:16:39.4742435Z                         time:   [4.3042 ms 4.3359 ms 4.3739 ms]
2026-08-02T22:16:39.4742841Z                         thrpt:  [11.432 Melem/s 11.532 Melem/s 11.617 Melem/s]
2026-08-02T22:16:39.4743229Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:16:39.4743535Z   1 (5.00%) high mild
2026-08-02T22:16:39.4747467Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue
2026-08-02T22:16:39.4748213Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:16:40.3289460Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.0081 s (1800 iterations)
2026-08-02T22:16:43.3450575Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue: Analyzing
2026-08-02T22:16:43.3776068Z full_cycle/1x1/1key_10000msg/DropValue
2026-08-02T22:16:43.3776685Z                         time:   [1.6724 ms 1.6744 ms 1.6772 ms]
2026-08-02T22:16:43.3777315Z                         thrpt:  [29.813 Melem/s 29.863 Melem/s 29.898 Melem/s]
2026-08-02T22:16:43.3777988Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:16:43.3778804Z   1 (5.00%) high severe
2026-08-02T22:16:43.3779600Z Benchmarking full_cycle/1x1/10key_1000msg/u64
2026-08-02T22:16:43.3780278Z Benchmarking full_cycle/1x1/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:16:44.2235764Z Benchmarking full_cycle/1x1/10key_1000msg/u64: Collecting 20 samples in estimated 3.0108 s (1820 iterations)
2026-08-02T22:16:47.1902967Z Benchmarking full_cycle/1x1/10key_1000msg/u64: Analyzing
2026-08-02T22:16:47.2186183Z full_cycle/1x1/10key_1000msg/u64
2026-08-02T22:16:47.2186730Z                         time:   [1.6240 ms 1.6289 ms 1.6347 ms]
2026-08-02T22:16:47.2187362Z                         thrpt:  [30.599 Melem/s 30.708 Melem/s 30.801 Melem/s]
2026-08-02T22:16:47.2188036Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:16:47.2188596Z   4 (20.00%) low severe
2026-08-02T22:16:47.2188959Z   1 (5.00%) high severe
2026-08-02T22:16:47.2190842Z Benchmarking full_cycle/1x1/10key_1000msg/String
2026-08-02T22:16:47.2191791Z Benchmarking full_cycle/1x1/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:16:47.8016624Z Benchmarking full_cycle/1x1/10key_1000msg/String: Collecting 20 samples in estimated 3.0263 s (660 iterations)
2026-08-02T22:16:50.8484954Z Benchmarking full_cycle/1x1/10key_1000msg/String: Analyzing
2026-08-02T22:16:50.8776066Z full_cycle/1x1/10key_1000msg/String
2026-08-02T22:16:50.8778275Z                         time:   [4.5544 ms 4.6150 ms 4.6750 ms]
2026-08-02T22:16:50.8778981Z                         thrpt:  [10.699 Melem/s 10.839 Melem/s 10.983 Melem/s]
2026-08-02T22:16:50.8780458Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue
2026-08-02T22:16:50.8781087Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:16:51.7326756Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0098 s (1800 iterations)
2026-08-02T22:16:54.9422340Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue: Analyzing
2026-08-02T22:16:54.9710146Z full_cycle/1x1/10key_1000msg/DropValue
2026-08-02T22:16:54.9710742Z                         time:   [1.7319 ms 1.7819 ms 1.8328 ms]
2026-08-02T22:16:54.9711643Z                         thrpt:  [27.292 Melem/s 28.072 Melem/s 28.882 Melem/s]
2026-08-02T22:16:54.9714355Z Benchmarking full_cycle/1x1/100key_100msg/u64
2026-08-02T22:16:54.9715042Z Benchmarking full_cycle/1x1/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:16:55.8698180Z Benchmarking full_cycle/1x1/100key_100msg/u64: Collecting 20 samples in estimated 3.0233 s (1720 iterations)
2026-08-02T22:16:58.8734283Z Benchmarking full_cycle/1x1/100key_100msg/u64: Analyzing
2026-08-02T22:16:58.9022075Z full_cycle/1x1/100key_100msg/u64
2026-08-02T22:16:58.9022448Z                         time:   [1.7418 ms 1.7450 ms 1.7477 ms]
2026-08-02T22:16:58.9022858Z                         thrpt:  [28.724 Melem/s 28.768 Melem/s 28.820 Melem/s]
2026-08-02T22:16:58.9023236Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:16:58.9023538Z   1 (5.00%) low severe
2026-08-02T22:16:58.9023755Z   2 (10.00%) low mild
2026-08-02T22:16:58.9025705Z Benchmarking full_cycle/1x1/100key_100msg/String
2026-08-02T22:16:58.9026309Z Benchmarking full_cycle/1x1/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:16:59.4969694Z Benchmarking full_cycle/1x1/100key_100msg/String: Collecting 20 samples in estimated 3.0881 s (660 iterations)
2026-08-02T22:17:02.5707288Z Benchmarking full_cycle/1x1/100key_100msg/String: Analyzing
2026-08-02T22:17:02.6003906Z full_cycle/1x1/100key_100msg/String
2026-08-02T22:17:02.6004496Z                         time:   [4.5964 ms 4.6557 ms 4.7142 ms]
2026-08-02T22:17:02.6005156Z                         thrpt:  [10.649 Melem/s 10.782 Melem/s 10.922 Melem/s]
2026-08-02T22:17:02.6008173Z Benchmarking full_cycle/1x1/100key_100msg/DropValue
2026-08-02T22:17:02.6008790Z Benchmarking full_cycle/1x1/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:17:03.5184252Z Benchmarking full_cycle/1x1/100key_100msg/DropValue: Collecting 20 samples in estimated 3.0161 s (1680 iterations)
2026-08-02T22:17:06.5492119Z Benchmarking full_cycle/1x1/100key_100msg/DropValue: Analyzing
2026-08-02T22:17:06.5778221Z full_cycle/1x1/100key_100msg/DropValue
2026-08-02T22:17:06.5778709Z                         time:   [1.7802 ms 1.8027 ms 1.8427 ms]
2026-08-02T22:17:06.5779101Z                         thrpt:  [27.242 Melem/s 27.847 Melem/s 28.200 Melem/s]
2026-08-02T22:17:06.5779499Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:17:06.5779799Z   2 (10.00%) high mild
2026-08-02T22:17:06.5780015Z   2 (10.00%) high severe
2026-08-02T22:17:06.5784232Z Benchmarking full_cycle/1x1/1000key_10msg/u64
2026-08-02T22:17:06.5784973Z Benchmarking full_cycle/1x1/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:17:07.1099080Z Benchmarking full_cycle/1x1/1000key_10msg/u64: Collecting 20 samples in estimated 3.0005 s (1440 iterations)
2026-08-02T22:17:10.1092283Z Benchmarking full_cycle/1x1/1000key_10msg/u64: Analyzing
2026-08-02T22:17:10.1393645Z full_cycle/1x1/1000key_10msg/u64
2026-08-02T22:17:10.1394078Z                         time:   [2.0788 ms 2.0815 ms 2.0849 ms]
2026-08-02T22:17:10.1394466Z                         thrpt:  [24.941 Melem/s 24.982 Melem/s 25.015 Melem/s]
2026-08-02T22:17:10.1394851Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:17:10.1395149Z   1 (5.00%) low mild
2026-08-02T22:17:10.1395382Z   1 (5.00%) high severe
2026-08-02T22:17:10.1397512Z Benchmarking full_cycle/1x1/1000key_10msg/String
2026-08-02T22:17:10.1398127Z Benchmarking full_cycle/1x1/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:17:10.7818333Z Benchmarking full_cycle/1x1/1000key_10msg/String: Collecting 20 samples in estimated 3.0326 s (600 iterations)
2026-08-02T22:17:13.6957107Z Benchmarking full_cycle/1x1/1000key_10msg/String: Analyzing
2026-08-02T22:17:13.7250295Z full_cycle/1x1/1000key_10msg/String
2026-08-02T22:17:13.7250895Z                         time:   [4.8298 ms 4.8549 ms 4.8822 ms]
2026-08-02T22:17:13.7251866Z                         thrpt:  [10.651 Melem/s 10.711 Melem/s 10.767 Melem/s]
2026-08-02T22:17:13.7252532Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:13.7253029Z   1 (5.00%) high mild
2026-08-02T22:17:13.7256356Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue
2026-08-02T22:17:13.7257145Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:17:14.2773904Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0283 s (1400 iterations)
2026-08-02T22:17:17.3119536Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue: Analyzing
2026-08-02T22:17:17.3408990Z full_cycle/1x1/1000key_10msg/DropValue
2026-08-02T22:17:17.3409600Z                         time:   [2.1631 ms 2.1663 ms 2.1697 ms]
2026-08-02T22:17:17.3410321Z                         thrpt:  [23.966 Melem/s 24.004 Melem/s 24.040 Melem/s]
2026-08-02T22:17:17.3412987Z Benchmarking full_cycle/1x1/10000key_1msg/u64
2026-08-02T22:17:17.3413693Z Benchmarking full_cycle/1x1/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:17:17.9978487Z Benchmarking full_cycle/1x1/10000key_1msg/u64: Collecting 20 samples in estimated 3.1009 s (600 iterations)
2026-08-02T22:17:21.0839607Z Benchmarking full_cycle/1x1/10000key_1msg/u64: Analyzing
2026-08-02T22:17:21.1130514Z full_cycle/1x1/10000key_1msg/u64
2026-08-02T22:17:21.1131077Z                         time:   [5.1355 ms 5.1419 ms 5.1484 ms]
2026-08-02T22:17:21.1132011Z                         thrpt:  [13.596 Melem/s 13.614 Melem/s 13.631 Melem/s]
2026-08-02T22:17:21.1132680Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:21.1133149Z   1 (5.00%) high mild
2026-08-02T22:17:21.1135478Z Benchmarking full_cycle/1x1/10000key_1msg/String
2026-08-02T22:17:21.1136280Z Benchmarking full_cycle/1x1/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:17:22.1041478Z Benchmarking full_cycle/1x1/10000key_1msg/String: Collecting 20 samples in estimated 3.1194 s (400 iterations)
2026-08-02T22:17:25.2302760Z Benchmarking full_cycle/1x1/10000key_1msg/String: Analyzing
2026-08-02T22:17:25.2593802Z full_cycle/1x1/10000key_1msg/String
2026-08-02T22:17:25.2594200Z                         time:   [7.8048 ms 7.8135 ms 7.8214 ms]
2026-08-02T22:17:25.2594586Z                         thrpt:  [8.9498 Melem/s 8.9588 Melem/s 8.9688 Melem/s]
2026-08-02T22:17:25.2594982Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:25.2595272Z   1 (5.00%) low mild
2026-08-02T22:17:25.2597244Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue
2026-08-02T22:17:25.2597880Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:17:25.9280517Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.0512 s (580 iterations)
2026-08-02T22:17:28.9749047Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue: Analyzing
2026-08-02T22:17:29.0034164Z full_cycle/1x1/10000key_1msg/DropValue
2026-08-02T22:17:29.0034760Z                         time:   [5.2438 ms 5.2516 ms 5.2592 ms]
2026-08-02T22:17:29.0035459Z                         thrpt:  [13.310 Melem/s 13.329 Melem/s 13.349 Melem/s]
2026-08-02T22:17:29.0038073Z Benchmarking full_cycle/16x1/1key_10000msg/u64
2026-08-02T22:17:29.0038697Z Benchmarking full_cycle/16x1/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:17:29.8279838Z Benchmarking full_cycle/16x1/1key_10000msg/u64: Collecting 20 samples in estimated 3.0316 s (1880 iterations)
2026-08-02T22:17:32.8544464Z Benchmarking full_cycle/16x1/1key_10000msg/u64: Analyzing
2026-08-02T22:17:32.8854585Z full_cycle/16x1/1key_10000msg/u64
2026-08-02T22:17:32.8855180Z                         time:   [1.6057 ms 1.6086 ms 1.6110 ms]
2026-08-02T22:17:32.8855850Z                         thrpt:  [31.038 Melem/s 31.084 Melem/s 31.141 Melem/s]
2026-08-02T22:17:32.8856826Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:17:32.8857317Z   2 (10.00%) low severe
2026-08-02T22:17:32.8860096Z Benchmarking full_cycle/16x1/1key_10000msg/String
2026-08-02T22:17:32.8860888Z Benchmarking full_cycle/16x1/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:17:33.4782617Z Benchmarking full_cycle/16x1/1key_10000msg/String: Collecting 20 samples in estimated 3.0770 s (660 iterations)
2026-08-02T22:17:36.4350707Z Benchmarking full_cycle/16x1/1key_10000msg/String: Analyzing
2026-08-02T22:17:36.4666668Z full_cycle/16x1/1key_10000msg/String
2026-08-02T22:17:36.4667030Z                         time:   [4.4597 ms 4.4786 ms 4.5065 ms]
2026-08-02T22:17:36.4667415Z                         thrpt:  [11.096 Melem/s 11.165 Melem/s 11.212 Melem/s]
2026-08-02T22:17:36.4667793Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:36.4668080Z   1 (5.00%) high severe
2026-08-02T22:17:36.4672516Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue
2026-08-02T22:17:36.4673213Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:17:37.3222928Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.0113 s (1800 iterations)
2026-08-02T22:17:40.3560224Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue: Analyzing
2026-08-02T22:17:40.3879370Z full_cycle/16x1/1key_10000msg/DropValue
2026-08-02T22:17:40.3880002Z                         time:   [1.6769 ms 1.6841 ms 1.6975 ms]
2026-08-02T22:17:40.3880667Z                         thrpt:  [29.457 Melem/s 29.690 Melem/s 29.818 Melem/s]
2026-08-02T22:17:40.3881558Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:40.3882048Z   1 (5.00%) high severe
2026-08-02T22:17:40.3885693Z Benchmarking full_cycle/16x1/10key_1000msg/u64
2026-08-02T22:17:40.3886409Z Benchmarking full_cycle/16x1/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:17:41.2151196Z Benchmarking full_cycle/16x1/10key_1000msg/u64: Collecting 20 samples in estimated 3.0079 s (1860 iterations)
2026-08-02T22:17:44.2093657Z Benchmarking full_cycle/16x1/10key_1000msg/u64: Analyzing
2026-08-02T22:17:44.2409882Z full_cycle/16x1/10key_1000msg/u64
2026-08-02T22:17:44.2410479Z                         time:   [1.6052 ms 1.6087 ms 1.6117 ms]
2026-08-02T22:17:44.2411747Z                         thrpt:  [31.035 Melem/s 31.093 Melem/s 31.162 Melem/s]
2026-08-02T22:17:44.2412525Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:17:44.2413079Z   4 (20.00%) low severe
2026-08-02T22:17:44.2414352Z Benchmarking full_cycle/16x1/10key_1000msg/String
2026-08-02T22:17:44.2415118Z Benchmarking full_cycle/16x1/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:17:44.8365607Z Benchmarking full_cycle/16x1/10key_1000msg/String: Collecting 20 samples in estimated 3.0917 s (660 iterations)
2026-08-02T22:17:47.9185462Z Benchmarking full_cycle/16x1/10key_1000msg/String: Analyzing
2026-08-02T22:17:47.9487217Z full_cycle/16x1/10key_1000msg/String
2026-08-02T22:17:47.9487872Z                         time:   [4.5379 ms 4.6683 ms 4.8436 ms]
2026-08-02T22:17:47.9488545Z                         thrpt:  [10.327 Melem/s 10.715 Melem/s 11.023 Melem/s]
2026-08-02T22:17:47.9489176Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:17:47.9489655Z   1 (5.00%) high mild
2026-08-02T22:17:47.9489996Z   2 (10.00%) high severe
2026-08-02T22:17:47.9495478Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue
2026-08-02T22:17:47.9496257Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:17:48.8395044Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0299 s (1740 iterations)
2026-08-02T22:17:51.7583763Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue: Analyzing
2026-08-02T22:17:51.7873600Z full_cycle/16x1/10key_1000msg/DropValue
2026-08-02T22:17:51.7874041Z                         time:   [1.6757 ms 1.6762 ms 1.6769 ms]
2026-08-02T22:17:51.7874420Z                         thrpt:  [29.829 Melem/s 29.840 Melem/s 29.851 Melem/s]
2026-08-02T22:17:51.7875080Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:51.7875377Z   1 (5.00%) high mild
2026-08-02T22:17:51.7877436Z Benchmarking full_cycle/16x1/100key_100msg/u64
2026-08-02T22:17:51.7878041Z Benchmarking full_cycle/16x1/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:17:52.6975077Z Benchmarking full_cycle/16x1/100key_100msg/u64: Collecting 20 samples in estimated 3.0261 s (1700 iterations)
2026-08-02T22:17:55.6599845Z Benchmarking full_cycle/16x1/100key_100msg/u64: Analyzing
2026-08-02T22:17:55.6904104Z full_cycle/16x1/100key_100msg/u64
2026-08-02T22:17:55.6904614Z                         time:   [1.7392 ms 1.7414 ms 1.7432 ms]
2026-08-02T22:17:55.6905242Z                         thrpt:  [28.797 Melem/s 28.827 Melem/s 28.863 Melem/s]
2026-08-02T22:17:55.6905668Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:55.6905962Z   1 (5.00%) low severe
2026-08-02T22:17:55.6908392Z Benchmarking full_cycle/16x1/100key_100msg/String
2026-08-02T22:17:55.6909030Z Benchmarking full_cycle/16x1/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:17:56.3059723Z Benchmarking full_cycle/16x1/100key_100msg/String: Collecting 20 samples in estimated 3.0021 s (620 iterations)
2026-08-02T22:17:59.2848218Z Benchmarking full_cycle/16x1/100key_100msg/String: Analyzing
2026-08-02T22:17:59.3134913Z full_cycle/16x1/100key_100msg/String
2026-08-02T22:17:59.3135487Z                         time:   [4.7423 ms 4.8028 ms 4.8790 ms]
2026-08-02T22:17:59.3136113Z                         thrpt:  [10.289 Melem/s 10.452 Melem/s 10.586 Melem/s]
2026-08-02T22:17:59.3136784Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:17:59.3137279Z   1 (5.00%) high severe
2026-08-02T22:17:59.3138142Z Benchmarking full_cycle/16x1/100key_100msg/DropValue
2026-08-02T22:17:59.3138893Z Benchmarking full_cycle/16x1/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:18:00.2480171Z Benchmarking full_cycle/16x1/100key_100msg/DropValue: Collecting 20 samples in estimated 3.0340 s (1660 iterations)
2026-08-02T22:18:03.2702532Z Benchmarking full_cycle/16x1/100key_100msg/DropValue: Analyzing
2026-08-02T22:18:03.3017598Z full_cycle/16x1/100key_100msg/DropValue
2026-08-02T22:18:03.3018455Z                         time:   [1.8180 ms 1.8194 ms 1.8209 ms]
2026-08-02T22:18:03.3019095Z                         thrpt:  [27.568 Melem/s 27.592 Melem/s 27.612 Melem/s]
2026-08-02T22:18:03.3019773Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:18:03.3020266Z   1 (5.00%) high mild
2026-08-02T22:18:03.3021478Z Benchmarking full_cycle/16x1/1000key_10msg/u64
2026-08-02T22:18:03.3022179Z Benchmarking full_cycle/16x1/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:18:03.8456805Z Benchmarking full_cycle/16x1/1000key_10msg/u64: Collecting 20 samples in estimated 3.0255 s (1420 iterations)
2026-08-02T22:18:06.8600446Z Benchmarking full_cycle/16x1/1000key_10msg/u64: Analyzing
2026-08-02T22:18:06.8886509Z full_cycle/16x1/1000key_10msg/u64
2026-08-02T22:18:06.8887083Z                         time:   [2.1179 ms 2.1216 ms 2.1250 ms]
2026-08-02T22:18:06.8887707Z                         thrpt:  [24.471 Melem/s 24.510 Melem/s 24.552 Melem/s]
2026-08-02T22:18:06.8888393Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:18:06.8888888Z   1 (5.00%) low mild
2026-08-02T22:18:06.8890581Z Benchmarking full_cycle/16x1/1000key_10msg/String
2026-08-02T22:18:06.8891497Z Benchmarking full_cycle/16x1/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:18:07.5479391Z Benchmarking full_cycle/16x1/1000key_10msg/String: Collecting 20 samples in estimated 3.0082 s (580 iterations)
2026-08-02T22:18:10.4895110Z Benchmarking full_cycle/16x1/1000key_10msg/String: Analyzing
2026-08-02T22:18:10.5181535Z full_cycle/16x1/1000key_10msg/String
2026-08-02T22:18:10.5182283Z                         time:   [5.0475 ms 5.0700 ms 5.0931 ms]
2026-08-02T22:18:10.5182743Z                         thrpt:  [10.210 Melem/s 10.256 Melem/s 10.302 Melem/s]
2026-08-02T22:18:10.5185813Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:18:10.5186246Z   1 (5.00%) high mild
2026-08-02T22:18:10.5186675Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue
2026-08-02T22:18:10.5187370Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:18:11.0825276Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0066 s (1360 iterations)
2026-08-02T22:18:14.1502207Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue: Analyzing
2026-08-02T22:18:14.1790624Z full_cycle/16x1/1000key_10msg/DropValue
2026-08-02T22:18:14.1791211Z                         time:   [2.2360 ms 2.2542 ms 2.2739 ms]
2026-08-02T22:18:14.1792025Z                         thrpt:  [22.868 Melem/s 23.068 Melem/s 23.256 Melem/s]
2026-08-02T22:18:14.1794670Z Benchmarking full_cycle/16x1/10000key_1msg/u64
2026-08-02T22:18:14.1795358Z Benchmarking full_cycle/16x1/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:18:14.8467859Z Benchmarking full_cycle/16x1/10000key_1msg/u64: Collecting 20 samples in estimated 3.0467 s (580 iterations)
2026-08-02T22:18:17.8681776Z Benchmarking full_cycle/16x1/10000key_1msg/u64: Analyzing
2026-08-02T22:18:17.8968627Z full_cycle/16x1/10000key_1msg/u64
2026-08-02T22:18:17.8969183Z                         time:   [5.1905 ms 5.2072 ms 5.2255 ms]
2026-08-02T22:18:17.8969803Z                         thrpt:  [13.396 Melem/s 13.443 Melem/s 13.486 Melem/s]
2026-08-02T22:18:17.8973540Z Benchmarking full_cycle/16x1/10000key_1msg/String
2026-08-02T22:18:17.8974416Z Benchmarking full_cycle/16x1/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:18:18.4020453Z Benchmarking full_cycle/16x1/10000key_1msg/String: Collecting 20 samples in estimated 3.0428 s (380 iterations)
2026-08-02T22:18:21.4270552Z Benchmarking full_cycle/16x1/10000key_1msg/String: Analyzing
2026-08-02T22:18:21.4559137Z full_cycle/16x1/10000key_1msg/String
2026-08-02T22:18:21.4559680Z                         time:   [7.9499 ms 7.9586 ms 7.9670 ms]
2026-08-02T22:18:21.4560288Z                         thrpt:  [8.7863 Melem/s 8.7955 Melem/s 8.8052 Melem/s]
2026-08-02T22:18:21.4563510Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue
2026-08-02T22:18:21.4564343Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:18:22.1274265Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.0637 s (580 iterations)
2026-08-02T22:18:25.1771730Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue: Analyzing
2026-08-02T22:18:25.2102942Z full_cycle/16x1/10000key_1msg/DropValue
2026-08-02T22:18:25.2103477Z                         time:   [5.2527 ms 5.2566 ms 5.2611 ms]
2026-08-02T22:18:25.2103996Z                         thrpt:  [13.305 Melem/s 13.317 Melem/s 13.326 Melem/s]
2026-08-02T22:18:25.2104388Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:18:25.2104682Z   2 (10.00%) high mild
2026-08-02T22:18:25.2107500Z Benchmarking full_cycle/1x16/1key_10000msg/u64
2026-08-02T22:18:25.2108116Z Benchmarking full_cycle/1x16/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:18:25.8099727Z Benchmarking full_cycle/1x16/1key_10000msg/u64: Collecting 20 samples in estimated 3.0426 s (320 iterations)
2026-08-02T22:18:28.8352443Z Benchmarking full_cycle/1x16/1key_10000msg/u64: Analyzing
2026-08-02T22:18:28.8650231Z full_cycle/1x16/1key_10000msg/u64
2026-08-02T22:18:28.8650795Z                         time:   [9.3904 ms 9.4524 ms 9.5021 ms]
2026-08-02T22:18:28.8651648Z                         thrpt:  [36.837 Melem/s 37.031 Melem/s 37.275 Melem/s]
2026-08-02T22:18:28.8652294Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:18:28.8652839Z   4 (20.00%) low severe
2026-08-02T22:18:28.8654227Z Benchmarking full_cycle/1x16/1key_10000msg/String
2026-08-02T22:18:28.8655006Z Benchmarking full_cycle/1x16/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:18:29.4153189Z Benchmarking full_cycle/1x16/1key_10000msg/String: Collecting 20 samples in estimated 3.1917 s (180 iterations)
2026-08-02T22:18:32.5716450Z Benchmarking full_cycle/1x16/1key_10000msg/String: Analyzing
2026-08-02T22:18:32.6005477Z full_cycle/1x16/1key_10000msg/String
2026-08-02T22:18:32.6006018Z                         time:   [17.487 ms 17.533 ms 17.575 ms]
2026-08-02T22:18:32.6006641Z                         thrpt:  [19.917 Melem/s 19.964 Melem/s 20.016 Melem/s]
2026-08-02T22:18:32.6007038Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:18:32.6007348Z   2 (10.00%) low severe
2026-08-02T22:18:32.6007589Z   1 (5.00%) low mild
2026-08-02T22:18:32.6007793Z   1 (5.00%) high mild
2026-08-02T22:18:32.6010805Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue
2026-08-02T22:18:32.6011793Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:18:33.2281794Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.1845 s (320 iterations)
2026-08-02T22:18:36.4215529Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue: Analyzing
2026-08-02T22:18:36.4508262Z full_cycle/1x16/1key_10000msg/DropValue
2026-08-02T22:18:36.4508836Z                         time:   [9.9700 ms 9.9770 ms 9.9836 ms]
2026-08-02T22:18:36.4509449Z                         thrpt:  [35.061 Melem/s 35.084 Melem/s 35.108 Melem/s]
2026-08-02T22:18:36.4510149Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:18:36.4510662Z   1 (5.00%) low mild
2026-08-02T22:18:36.4511477Z Benchmarking full_cycle/1x16/10key_1000msg/u64
2026-08-02T22:18:36.4512172Z Benchmarking full_cycle/1x16/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:18:37.0467838Z Benchmarking full_cycle/1x16/10key_1000msg/u64: Collecting 20 samples in estimated 3.0245 s (320 iterations)
2026-08-02T22:18:40.0573902Z Benchmarking full_cycle/1x16/10key_1000msg/u64: Analyzing
2026-08-02T22:18:40.0864416Z full_cycle/1x16/10key_1000msg/u64
2026-08-02T22:18:40.0864966Z                         time:   [9.3469 ms 9.4062 ms 9.4546 ms]
2026-08-02T22:18:40.0865593Z                         thrpt:  [37.053 Melem/s 37.244 Melem/s 37.480 Melem/s]
2026-08-02T22:18:40.0866266Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:18:40.0866810Z   4 (20.00%) low severe
2026-08-02T22:18:40.0867166Z   1 (5.00%) high severe
2026-08-02T22:18:40.0868830Z Benchmarking full_cycle/1x16/10key_1000msg/String
2026-08-02T22:18:40.0869688Z Benchmarking full_cycle/1x16/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:18:40.6373900Z Benchmarking full_cycle/1x16/10key_1000msg/String: Collecting 20 samples in estimated 3.1954 s (180 iterations)
2026-08-02T22:18:43.7813344Z Benchmarking full_cycle/1x16/10key_1000msg/String: Analyzing
2026-08-02T22:18:43.8108780Z full_cycle/1x16/10key_1000msg/String
2026-08-02T22:18:43.8109348Z                         time:   [17.412 ms 17.464 ms 17.536 ms]
2026-08-02T22:18:43.8109975Z                         thrpt:  [19.977 Melem/s 20.060 Melem/s 20.119 Melem/s]
2026-08-02T22:18:43.8110609Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:18:43.8111140Z   1 (5.00%) high severe
2026-08-02T22:18:43.8122267Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue
2026-08-02T22:18:43.8123046Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:18:44.4462628Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0231 s (300 iterations)
2026-08-02T22:18:47.4744433Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue: Analyzing
2026-08-02T22:18:47.5033508Z full_cycle/1x16/10key_1000msg/DropValue
2026-08-02T22:18:47.5034067Z                         time:   [9.9364 ms 10.092 ms 10.316 ms]
2026-08-02T22:18:47.5034455Z                         thrpt:  [33.958 Melem/s 34.712 Melem/s 35.256 Melem/s]
2026-08-02T22:18:47.5034839Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:18:47.5035134Z   3 (15.00%) high severe
2026-08-02T22:18:47.5036736Z Benchmarking full_cycle/1x16/100key_100msg/u64
2026-08-02T22:18:47.5037291Z Benchmarking full_cycle/1x16/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:18:48.0960703Z Benchmarking full_cycle/1x16/100key_100msg/u64: Collecting 20 samples in estimated 3.0080 s (320 iterations)
2026-08-02T22:18:51.1076372Z Benchmarking full_cycle/1x16/100key_100msg/u64: Analyzing
2026-08-02T22:18:51.1357302Z full_cycle/1x16/100key_100msg/u64
2026-08-02T22:18:51.1357890Z                         time:   [9.3541 ms 9.4093 ms 9.4892 ms]
2026-08-02T22:18:51.1358283Z                         thrpt:  [37.221 Melem/s 37.538 Melem/s 37.759 Melem/s]
2026-08-02T22:18:51.1358660Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:18:51.1358979Z   3 (15.00%) low severe
2026-08-02T22:18:51.1359201Z   1 (5.00%) high mild
2026-08-02T22:18:51.1359412Z   1 (5.00%) high severe
2026-08-02T22:18:51.1360235Z Benchmarking full_cycle/1x16/100key_100msg/String
2026-08-02T22:18:51.1360674Z Benchmarking full_cycle/1x16/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:18:51.6872167Z Benchmarking full_cycle/1x16/100key_100msg/String: Collecting 20 samples in estimated 3.1994 s (180 iterations)
2026-08-02T22:18:54.8576159Z Benchmarking full_cycle/1x16/100key_100msg/String: Analyzing
2026-08-02T22:18:54.8867722Z full_cycle/1x16/100key_100msg/String
2026-08-02T22:18:54.8868262Z                         time:   [17.572 ms 17.610 ms 17.641 ms]
2026-08-02T22:18:54.8868757Z                         thrpt:  [20.021 Melem/s 20.057 Melem/s 20.101 Melem/s]
2026-08-02T22:18:54.8869156Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:18:54.8869452Z   3 (15.00%) low severe
2026-08-02T22:18:54.8871619Z Benchmarking full_cycle/1x16/100key_100msg/DropValue
2026-08-02T22:18:54.8872663Z Benchmarking full_cycle/1x16/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:18:55.5154151Z Benchmarking full_cycle/1x16/100key_100msg/DropValue: Collecting 20 samples in estimated 3.1904 s (320 iterations)
2026-08-02T22:18:58.7111237Z Benchmarking full_cycle/1x16/100key_100msg/DropValue: Analyzing
2026-08-02T22:18:58.7427125Z full_cycle/1x16/100key_100msg/DropValue
2026-08-02T22:18:58.7427693Z                         time:   [9.9794 ms 9.9848 ms 9.9905 ms]
2026-08-02T22:18:58.7428225Z                         thrpt:  [35.354 Melem/s 35.374 Melem/s 35.393 Melem/s]
2026-08-02T22:18:58.7431619Z Benchmarking full_cycle/1x16/1000key_10msg/u64
2026-08-02T22:18:58.7432641Z Benchmarking full_cycle/1x16/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:18:59.4133905Z Benchmarking full_cycle/1x16/1000key_10msg/u64: Collecting 20 samples in estimated 3.1906 s (300 iterations)
2026-08-02T22:19:02.5962639Z Benchmarking full_cycle/1x16/1000key_10msg/u64: Analyzing
2026-08-02T22:19:02.6247455Z full_cycle/1x16/1000key_10msg/u64
2026-08-02T22:19:02.6247974Z                         time:   [10.575 ms 10.608 ms 10.638 ms]
2026-08-02T22:19:02.6248523Z                         thrpt:  [35.909 Melem/s 36.011 Melem/s 36.123 Melem/s]
2026-08-02T22:19:02.6248917Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:19:02.6249253Z   4 (20.00%) low severe
2026-08-02T22:19:02.6249486Z   1 (5.00%) high severe
2026-08-02T22:19:02.6250132Z Benchmarking full_cycle/1x16/1000key_10msg/String
2026-08-02T22:19:02.6250844Z Benchmarking full_cycle/1x16/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:19:03.2137312Z Benchmarking full_cycle/1x16/1000key_10msg/String: Collecting 20 samples in estimated 3.0373 s (160 iterations)
2026-08-02T22:19:06.2380010Z Benchmarking full_cycle/1x16/1000key_10msg/String: Analyzing
2026-08-02T22:19:06.2680291Z full_cycle/1x16/1000key_10msg/String
2026-08-02T22:19:06.2680845Z                         time:   [18.856 ms 18.899 ms 18.937 ms]
2026-08-02T22:19:06.2681496Z                         thrpt:  [20.172 Melem/s 20.212 Melem/s 20.258 Melem/s]
2026-08-02T22:19:06.2681895Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:19:06.2682192Z   4 (20.00%) low severe
2026-08-02T22:19:06.2682408Z   1 (5.00%) high mild
2026-08-02T22:19:06.2686552Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue
2026-08-02T22:19:06.2687313Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:19:06.9775461Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.1497 s (280 iterations)
2026-08-02T22:19:10.1346119Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue: Analyzing
2026-08-02T22:19:10.1660725Z full_cycle/1x16/1000key_10msg/DropValue
2026-08-02T22:19:10.1661478Z                         time:   [11.261 ms 11.273 ms 11.285 ms]
2026-08-02T22:19:10.1661887Z                         thrpt:  [33.851 Melem/s 33.885 Melem/s 33.922 Melem/s]
2026-08-02T22:19:10.1662278Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:19:10.1662568Z   1 (5.00%) low mild
2026-08-02T22:19:10.1665213Z Benchmarking full_cycle/1x16/10000key_1msg/u64
2026-08-02T22:19:10.1665902Z Benchmarking full_cycle/1x16/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:19:10.8841740Z Benchmarking full_cycle/1x16/10000key_1msg/u64: Collecting 20 samples in estimated 3.2400 s (140 iterations)
2026-08-02T22:19:14.1204983Z Benchmarking full_cycle/1x16/10000key_1msg/u64: Analyzing
2026-08-02T22:19:14.1500811Z full_cycle/1x16/10000key_1msg/u64
2026-08-02T22:19:14.1501555Z                         time:   [23.044 ms 23.113 ms 23.180 ms]
2026-08-02T22:19:14.1502195Z                         thrpt:  [28.904 Melem/s 28.988 Melem/s 29.075 Melem/s]
2026-08-02T22:19:14.1504491Z Benchmarking full_cycle/1x16/10000key_1msg/String
2026-08-02T22:19:14.1505217Z Benchmarking full_cycle/1x16/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:19:15.0811743Z Benchmarking full_cycle/1x16/10000key_1msg/String: Collecting 20 samples in estimated 3.0014 s (100 iterations)
2026-08-02T22:19:18.0882359Z Benchmarking full_cycle/1x16/10000key_1msg/String: Analyzing
2026-08-02T22:19:18.1190267Z full_cycle/1x16/10000key_1msg/String
2026-08-02T22:19:18.1190838Z                         time:   [30.017 ms 30.067 ms 30.112 ms]
2026-08-02T22:19:18.1191663Z                         thrpt:  [22.250 Melem/s 22.283 Melem/s 22.321 Melem/s]
2026-08-02T22:19:18.1192297Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:19:18.1192858Z   2 (10.00%) low mild
2026-08-02T22:19:18.1193965Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue
2026-08-02T22:19:18.1194704Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:19:18.8227816Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.1753 s (140 iterations)
2026-08-02T22:19:21.9942533Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue: Analyzing
2026-08-02T22:19:22.0253862Z full_cycle/1x16/10000key_1msg/DropValue
2026-08-02T22:19:22.0254420Z                         time:   [22.631 ms 22.651 ms 22.672 ms]
2026-08-02T22:19:22.0254913Z                         thrpt:  [29.552 Melem/s 29.579 Melem/s 29.606 Melem/s]
2026-08-02T22:19:22.0257917Z Benchmarking full_cycle/4x4/1key_10000msg/u64
2026-08-02T22:19:22.0258580Z Benchmarking full_cycle/4x4/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:19:22.8392201Z Benchmarking full_cycle/4x4/1key_10000msg/u64: Collecting 20 samples in estimated 3.0614 s (960 iterations)
2026-08-02T22:19:25.8909690Z Benchmarking full_cycle/4x4/1key_10000msg/u64: Analyzing
2026-08-02T22:19:25.9198077Z full_cycle/4x4/1key_10000msg/u64
2026-08-02T22:19:25.9198628Z                         time:   [3.1641 ms 3.1776 ms 3.1889 ms]
2026-08-02T22:19:25.9199252Z                         thrpt:  [34.497 Melem/s 34.619 Melem/s 34.768 Melem/s]
2026-08-02T22:19:25.9199886Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:19:25.9200328Z   4 (20.00%) low severe
2026-08-02T22:19:25.9200546Z   1 (5.00%) high mild
2026-08-02T22:19:25.9203772Z Benchmarking full_cycle/4x4/1key_10000msg/String
2026-08-02T22:19:25.9204210Z Benchmarking full_cycle/4x4/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:19:26.8299356Z Benchmarking full_cycle/4x4/1key_10000msg/String: Collecting 20 samples in estimated 3.0074 s (420 iterations)
2026-08-02T22:19:29.7523217Z Benchmarking full_cycle/4x4/1key_10000msg/String: Analyzing
2026-08-02T22:19:29.7822088Z full_cycle/4x4/1key_10000msg/String
2026-08-02T22:19:29.7822806Z                         time:   [6.9332 ms 6.9562 ms 6.9880 ms]
2026-08-02T22:19:29.7832750Z                         thrpt:  [15.742 Melem/s 15.814 Melem/s 15.867 Melem/s]
2026-08-02T22:19:29.7833197Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:19:29.7833707Z   2 (10.00%) high severe
2026-08-02T22:19:29.7834211Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue
2026-08-02T22:19:29.7834934Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:19:30.6315447Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.0623 s (920 iterations)
2026-08-02T22:19:33.7013434Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue: Analyzing
2026-08-02T22:19:33.7306297Z full_cycle/4x4/1key_10000msg/DropValue
2026-08-02T22:19:33.7306897Z                         time:   [3.3327 ms 3.3352 ms 3.3376 ms]
2026-08-02T22:19:33.7307563Z                         thrpt:  [32.960 Melem/s 32.984 Melem/s 33.009 Melem/s]
2026-08-02T22:19:33.7309976Z Benchmarking full_cycle/4x4/10key_1000msg/u64
2026-08-02T22:19:33.7310574Z Benchmarking full_cycle/4x4/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:19:34.5435504Z Benchmarking full_cycle/4x4/10key_1000msg/u64: Collecting 20 samples in estimated 3.0585 s (960 iterations)
2026-08-02T22:19:37.6052765Z Benchmarking full_cycle/4x4/10key_1000msg/u64: Analyzing
2026-08-02T22:19:37.6348031Z full_cycle/4x4/10key_1000msg/u64
2026-08-02T22:19:37.6348603Z                         time:   [3.1654 ms 3.1880 ms 3.2195 ms]
2026-08-02T22:19:37.6349241Z                         thrpt:  [34.192 Melem/s 34.530 Melem/s 34.776 Melem/s]
2026-08-02T22:19:37.6349873Z Found 7 outliers among 20 measurements (35.00%)
2026-08-02T22:19:37.6350420Z   4 (20.00%) low severe
2026-08-02T22:19:37.6350771Z   2 (10.00%) high mild
2026-08-02T22:19:37.6351115Z   1 (5.00%) high severe
2026-08-02T22:19:37.6352456Z Benchmarking full_cycle/4x4/10key_1000msg/String
2026-08-02T22:19:37.6352933Z Benchmarking full_cycle/4x4/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:19:38.5459305Z Benchmarking full_cycle/4x4/10key_1000msg/String: Collecting 20 samples in estimated 3.0112 s (420 iterations)
2026-08-02T22:19:41.5339003Z Benchmarking full_cycle/4x4/10key_1000msg/String: Analyzing
2026-08-02T22:19:41.5628791Z full_cycle/4x4/10key_1000msg/String
2026-08-02T22:19:41.5629358Z                         time:   [6.9805 ms 7.1125 ms 7.3121 ms]
2026-08-02T22:19:41.5629901Z                         thrpt:  [15.054 Melem/s 15.477 Melem/s 15.770 Melem/s]
2026-08-02T22:19:41.5630301Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:19:41.5630599Z   1 (5.00%) high mild
2026-08-02T22:19:41.5630812Z   2 (10.00%) high severe
2026-08-02T22:19:41.5633909Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue
2026-08-02T22:19:41.5634382Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:19:42.4086554Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0490 s (920 iterations)
2026-08-02T22:19:45.4906399Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue: Analyzing
2026-08-02T22:19:45.5197535Z full_cycle/4x4/10key_1000msg/DropValue
2026-08-02T22:19:45.5198162Z                         time:   [3.3383 ms 3.3485 ms 3.3596 ms]
2026-08-02T22:19:45.5198795Z                         thrpt:  [32.765 Melem/s 32.874 Melem/s 32.975 Melem/s]
2026-08-02T22:19:45.5199443Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:19:45.5199984Z   1 (5.00%) high mild
2026-08-02T22:19:45.5202881Z Benchmarking full_cycle/4x4/100key_100msg/u64
2026-08-02T22:19:45.5203805Z Benchmarking full_cycle/4x4/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:19:46.3580034Z Benchmarking full_cycle/4x4/100key_100msg/u64: Collecting 20 samples in estimated 3.0219 s (920 iterations)
2026-08-02T22:19:49.3677792Z Benchmarking full_cycle/4x4/100key_100msg/u64: Analyzing
2026-08-02T22:19:49.3984437Z full_cycle/4x4/100key_100msg/u64
2026-08-02T22:19:49.3985295Z                         time:   [3.2603 ms 3.2701 ms 3.2786 ms]
2026-08-02T22:19:49.3985736Z                         thrpt:  [33.795 Melem/s 33.883 Melem/s 33.985 Melem/s]
2026-08-02T22:19:49.3986169Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:19:49.3986588Z   4 (20.00%) low severe
2026-08-02T22:19:49.3987150Z Benchmarking full_cycle/4x4/100key_100msg/String
2026-08-02T22:19:49.3987977Z Benchmarking full_cycle/4x4/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:19:50.3214390Z Benchmarking full_cycle/4x4/100key_100msg/String: Collecting 20 samples in estimated 3.0510 s (420 iterations)
2026-08-02T22:19:53.3237230Z Benchmarking full_cycle/4x4/100key_100msg/String: Analyzing
2026-08-02T22:19:53.3529722Z full_cycle/4x4/100key_100msg/String
2026-08-02T22:19:53.3530279Z                         time:   [7.1345 ms 7.1464 ms 7.1645 ms]
2026-08-02T22:19:53.3530909Z                         thrpt:  [15.465 Melem/s 15.504 Melem/s 15.530 Melem/s]
2026-08-02T22:19:53.3531837Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:19:53.3532333Z   1 (5.00%) high mild
2026-08-02T22:19:53.3532727Z   1 (5.00%) high severe
2026-08-02T22:19:53.3535732Z Benchmarking full_cycle/4x4/100key_100msg/DropValue
2026-08-02T22:19:53.3536384Z Benchmarking full_cycle/4x4/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:19:54.2330672Z Benchmarking full_cycle/4x4/100key_100msg/DropValue: Collecting 20 samples in estimated 3.0345 s (880 iterations)
2026-08-02T22:19:57.2784099Z Benchmarking full_cycle/4x4/100key_100msg/DropValue: Analyzing
2026-08-02T22:19:57.3073338Z full_cycle/4x4/100key_100msg/DropValue
2026-08-02T22:19:57.3073897Z                         time:   [3.4499 ms 3.4591 ms 3.4689 ms]
2026-08-02T22:19:57.3074389Z                         thrpt:  [31.941 Melem/s 32.031 Melem/s 32.117 Melem/s]
2026-08-02T22:19:57.3077176Z Benchmarking full_cycle/4x4/1000key_10msg/u64
2026-08-02T22:19:57.3077910Z Benchmarking full_cycle/4x4/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:19:58.2948759Z Benchmarking full_cycle/4x4/1000key_10msg/u64: Collecting 20 samples in estimated 3.0191 s (780 iterations)
2026-08-02T22:20:01.2925227Z Benchmarking full_cycle/4x4/1000key_10msg/u64: Analyzing
2026-08-02T22:20:01.3214773Z full_cycle/4x4/1000key_10msg/u64
2026-08-02T22:20:01.3215327Z                         time:   [3.8328 ms 3.8418 ms 3.8497 ms]
2026-08-02T22:20:01.3215952Z                         thrpt:  [30.651 Melem/s 30.715 Melem/s 30.787 Melem/s]
2026-08-02T22:20:01.3216651Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:20:01.3217149Z   2 (10.00%) low mild
2026-08-02T22:20:01.3232068Z Benchmarking full_cycle/4x4/1000key_10msg/String
2026-08-02T22:20:01.3242274Z Benchmarking full_cycle/4x4/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:20:02.2999865Z Benchmarking full_cycle/4x4/1000key_10msg/String: Collecting 20 samples in estimated 3.0804 s (400 iterations)
2026-08-02T22:20:05.3684573Z Benchmarking full_cycle/4x4/1000key_10msg/String: Analyzing
2026-08-02T22:20:05.3974155Z full_cycle/4x4/1000key_10msg/String
2026-08-02T22:20:05.3974720Z                         time:   [7.6628 ms 7.6694 ms 7.6767 ms]
2026-08-02T22:20:05.3975162Z                         thrpt:  [15.371 Melem/s 15.386 Melem/s 15.399 Melem/s]
2026-08-02T22:20:05.3975580Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:20:05.3975883Z   1 (5.00%) high mild
2026-08-02T22:20:05.3978585Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue
2026-08-02T22:20:05.3979340Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:20:05.9102356Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0648 s (760 iterations)
2026-08-02T22:20:08.9834186Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue: Analyzing
2026-08-02T22:20:09.0124041Z full_cycle/4x4/1000key_10msg/DropValue
2026-08-02T22:20:09.0124579Z                         time:   [4.0378 ms 4.0423 ms 4.0476 ms]
2026-08-02T22:20:09.0127371Z                         thrpt:  [29.153 Melem/s 29.192 Melem/s 29.224 Melem/s]
2026-08-02T22:20:09.0128252Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:20:09.0129005Z   1 (5.00%) high mild
2026-08-02T22:20:09.0129970Z Benchmarking full_cycle/4x4/10000key_1msg/u64
2026-08-02T22:20:09.0130688Z Benchmarking full_cycle/4x4/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:20:09.5548520Z Benchmarking full_cycle/4x4/10000key_1msg/u64: Collecting 20 samples in estimated 3.0951 s (360 iterations)
2026-08-02T22:20:12.6581093Z Benchmarking full_cycle/4x4/10000key_1msg/u64: Analyzing
2026-08-02T22:20:12.6898949Z full_cycle/4x4/10000key_1msg/u64
2026-08-02T22:20:12.6899501Z                         time:   [8.6015 ms 8.6182 ms 8.6331 ms]
2026-08-02T22:20:12.6899939Z                         thrpt:  [22.008 Melem/s 22.046 Melem/s 22.089 Melem/s]
2026-08-02T22:20:12.6900331Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:20:12.6900649Z   1 (5.00%) low severe
2026-08-02T22:20:12.6900895Z   3 (15.00%) low mild
2026-08-02T22:20:12.6904260Z Benchmarking full_cycle/4x4/10000key_1msg/String
2026-08-02T22:20:12.6904874Z Benchmarking full_cycle/4x4/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:20:13.4823628Z Benchmarking full_cycle/4x4/10000key_1msg/String: Collecting 20 samples in estimated 3.0164 s (240 iterations)
2026-08-02T22:20:16.4722395Z Benchmarking full_cycle/4x4/10000key_1msg/String: Analyzing
2026-08-02T22:20:16.5011216Z full_cycle/4x4/10000key_1msg/String
2026-08-02T22:20:16.5011979Z                         time:   [12.435 ms 12.455 ms 12.475 ms]
2026-08-02T22:20:16.5012518Z                         thrpt:  [15.231 Melem/s 15.255 Melem/s 15.279 Melem/s]
2026-08-02T22:20:16.5015257Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue
2026-08-02T22:20:16.5015714Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:20:17.0514663Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.1414 s (360 iterations)
2026-08-02T22:20:20.1985795Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue: Analyzing
2026-08-02T22:20:20.2276933Z full_cycle/4x4/10000key_1msg/DropValue
2026-08-02T22:20:20.2277491Z                         time:   [8.7352 ms 8.7401 ms 8.7455 ms]
2026-08-02T22:20:20.2278420Z                         thrpt:  [21.726 Melem/s 21.739 Melem/s 21.751 Melem/s]
2026-08-02T22:20:20.2279113Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:20:20.2279603Z   1 (5.00%) high mild
2026-08-02T22:20:20.2281853Z 
2026-08-02T22:20:20.2283459Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10000key_1msg/DropValue,20.000,25.000,36.100,707
2026-08-02T22:20:20.2285784Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10000key_1msg/DropValue,157.853,162.453,167.956,707
2026-08-02T22:20:20.2287801Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10000key_1msg/DropValue,29.808,30.463,31.903,707
2026-08-02T22:20:20.2289202Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10000key_1msg/DropValue,24.297,24.757,26.009,707
2026-08-02T22:20:20.2290633Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10000key_1msg/DropValue,64.094,66.993,69.334,707
2026-08-02T22:20:20.2292882Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10000key_1msg/DropValue,51.961,54.429,56.896,707
2026-08-02T22:20:20.2294551Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10000key_1msg/DropValue,151.139,152.864,159.815,707
2026-08-02T22:20:20.2296177Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10000key_1msg/String,20.000,25.100,38.100,443
2026-08-02T22:20:20.2297439Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10000key_1msg/String,157.913,162.333,169.543,443
2026-08-02T22:20:20.2298675Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10000key_1msg/String,52.874,54.808,58.219,443
2026-08-02T22:20:20.2299953Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10000key_1msg/String,92.751,95.537,97.489,443
2026-08-02T22:20:20.2301510Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10000key_1msg/String,133.122,143.576,147.985,443
2026-08-02T22:20:20.2303283Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10000key_1msg/String,122.629,130.257,139.048,443
2026-08-02T22:20:20.2304647Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10000key_1msg/String,151.432,153.342,168.087,443
2026-08-02T22:20:20.2305934Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10000key_1msg/u64,19.000,26.000,46.100,707
2026-08-02T22:20:20.2307163Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10000key_1msg/u64,154.704,160.356,177.427,707
2026-08-02T22:20:20.2308381Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10000key_1msg/u64,28.117,28.983,31.230,707
2026-08-02T22:20:20.2312215Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10000key_1msg/u64,23.973,24.323,26.489,707
2026-08-02T22:20:20.2314524Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10000key_1msg/u64,64.666,66.600,70.954,707
2026-08-02T22:20:20.2316850Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10000key_1msg/u64,52.653,55.054,57.752,707
2026-08-02T22:20:20.2319161Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10000key_1msg/u64,147.520,149.142,154.971,707
2026-08-02T22:20:20.2320674Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1000key_10msg/DropValue,19.100,24.000,35.000,1615
2026-08-02T22:20:20.2322368Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1000key_10msg/DropValue,159.407,164.417,185.336,1615
2026-08-02T22:20:20.2323645Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1000key_10msg/DropValue,28.203,28.753,30.525,1615
2026-08-02T22:20:20.2324973Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1000key_10msg/DropValue,23.338,24.768,26.755,1615
2026-08-02T22:20:20.2326586Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1000key_10msg/DropValue,56.017,58.682,63.111,1615
2026-08-02T22:20:20.2328054Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1000key_10msg/DropValue,43.662,45.724,48.967,1615
2026-08-02T22:20:20.2329440Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1000key_10msg/DropValue,174.425,178.783,200.053,1615
2026-08-02T22:20:20.2330730Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1000key_10msg/String,20.000,24.000,30.000,707
2026-08-02T22:20:20.2332640Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1000key_10msg/String,161.630,166.760,178.613,707
2026-08-02T22:20:20.2333898Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1000key_10msg/String,52.512,61.031,69.860,707
2026-08-02T22:20:20.2335163Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1000key_10msg/String,92.504,94.486,96.278,707
2026-08-02T22:20:20.2336536Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1000key_10msg/String,126.367,136.615,140.345,707
2026-08-02T22:20:20.2337970Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1000key_10msg/String,116.337,122.757,130.906,707
2026-08-02T22:20:20.2339324Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1000key_10msg/String,174.795,178.983,195.444,707
2026-08-02T22:20:20.2340718Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1000key_10msg/u64,19.000,23.000,28.100,1675
2026-08-02T22:20:20.2343149Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1000key_10msg/u64,151.693,157.002,169.575,1675
2026-08-02T22:20:20.2344437Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1000key_10msg/u64,25.557,26.365,27.393,1675
2026-08-02T22:20:20.2345705Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1000key_10msg/u64,23.005,23.581,24.839,1675
2026-08-02T22:20:20.2347091Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1000key_10msg/u64,54.017,55.992,57.969,1675
2026-08-02T22:20:20.2348539Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1000key_10msg/u64,43.362,44.986,46.646,1675
2026-08-02T22:20:20.2349873Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1000key_10msg/u64,172.271,176.098,188.220,1675
2026-08-02T22:20:20.2351142Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/100key_100msg/DropValue,20.000,23.000,28.100,2171
2026-08-02T22:20:20.2353856Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/100key_100msg/DropValue,227.620,240.050,268.900,2171
2026-08-02T22:20:20.2355907Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/100key_100msg/DropValue,27.619,28.263,29.054,2171
2026-08-02T22:20:20.2357565Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/100key_100msg/DropValue,23.698,24.508,25.871,2171
2026-08-02T22:20:20.2359533Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/100key_100msg/DropValue,51.008,52.663,53.934,2171
2026-08-02T22:20:20.2361647Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/100key_100msg/DropValue,40.032,41.275,42.852,2171
2026-08-02T22:20:20.2363547Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/100key_100msg/DropValue,387.420,400.350,493.320,2171
2026-08-02T22:20:20.2365006Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/100key_100msg/String,19.100,23.100,33.100,747
2026-08-02T22:20:20.2366258Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/100key_100msg/String,240.450,254.880,349.750,747
2026-08-02T22:20:20.2367504Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/100key_100msg/String,52.096,59.072,68.235,747
2026-08-02T22:20:20.2368795Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/100key_100msg/String,94.200,99.222,145.421,747
2026-08-02T22:20:20.2370342Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/100key_100msg/String,121.312,132.175,135.430,747
2026-08-02T22:20:20.2372411Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/100key_100msg/String,112.360,119.564,126.690,747
2026-08-02T22:20:20.2374905Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/100key_100msg/String,398.550,417.380,516.560,747
2026-08-02T22:20:20.2377157Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/100key_100msg/u64,19.000,22.000,27.100,2211
2026-08-02T22:20:20.2379347Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/100key_100msg/u64,146.070,157.890,165.610,2211
2026-08-02T22:20:20.2380832Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/100key_100msg/u64,25.067,26.145,26.989,2211
2026-08-02T22:20:20.2382925Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/100key_100msg/u64,22.629,23.159,25.278,2211
2026-08-02T22:20:20.2385299Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/100key_100msg/u64,49.676,51.617,54.201,2211
2026-08-02T22:20:20.2387945Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/100key_100msg/u64,39.646,41.075,42.621,2211
2026-08-02T22:20:20.2390279Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/100key_100msg/u64,379.910,392.330,485.100,2211
2026-08-02T22:20:20.2392728Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10key_1000msg/DropValue,20.000,23.000,28.100,2251
2026-08-02T22:20:20.2394970Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10key_1000msg/DropValue,629.200,688.300,835.600,2251
2026-08-02T22:20:20.2397215Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10key_1000msg/DropValue,27.362,27.981,28.473,2251
2026-08-02T22:20:20.2399544Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10key_1000msg/DropValue,22.431,22.837,23.998,2251
2026-08-02T22:20:20.2402176Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10key_1000msg/DropValue,46.722,47.980,49.367,2251
2026-08-02T22:20:20.2404738Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10key_1000msg/DropValue,37.147,38.282,45.833,2251
2026-08-02T22:20:20.2407182Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10key_1000msg/DropValue,1636.000,1673.200,1733.200,2251
2026-08-02T22:20:20.2409472Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10key_1000msg/String,20.000,22.100,32.000,787
2026-08-02T22:20:20.2412080Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10key_1000msg/String,1048.900,1082.900,1352.500,787
2026-08-02T22:20:20.2414297Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10key_1000msg/String,50.674,58.071,69.350,787
2026-08-02T22:20:20.2416384Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10key_1000msg/String,91.834,95.180,102.723,787
2026-08-02T22:20:20.2417782Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10key_1000msg/String,116.428,126.958,133.472,787
2026-08-02T22:20:20.2419223Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10key_1000msg/String,110.397,120.746,129.298,787
2026-08-02T22:20:20.2420604Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10key_1000msg/String,1583.900,1632.000,1872.500,787
2026-08-02T22:20:20.2422713Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10key_1000msg/u64,19.000,22.000,27.000,2371
2026-08-02T22:20:20.2424980Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10key_1000msg/u64,132.200,147.300,175.300,2371
2026-08-02T22:20:20.2426980Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10key_1000msg/u64,24.719,25.870,26.466,2371
2026-08-02T22:20:20.2428231Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10key_1000msg/u64,22.186,22.489,23.556,2371
2026-08-02T22:20:20.2429581Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10key_1000msg/u64,45.180,46.023,47.335,2371
2026-08-02T22:20:20.2430971Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10key_1000msg/u64,37.166,37.911,39.173,2371
2026-08-02T22:20:20.2432575Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10key_1000msg/u64,1619.000,1656.100,1714.100,2371
2026-08-02T22:20:20.2433866Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1key_10000msg/DropValue,19.000,22.100,27.100,2311
2026-08-02T22:20:20.2435237Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1key_10000msg/DropValue,9648.000,10820.000,15418.000,2311
2026-08-02T22:20:20.2437582Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1key_10000msg/DropValue,27.386,27.977,28.527,2311
2026-08-02T22:20:20.2439991Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1key_10000msg/DropValue,22.388,22.766,23.936,2311
2026-08-02T22:20:20.2442748Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1key_10000msg/DropValue,46.440,47.381,48.232,2311
2026-08-02T22:20:20.2445392Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1key_10000msg/DropValue,36.903,37.414,39.110,2311
2026-08-02T22:20:20.2447992Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1key_10000msg/DropValue,22832.000,23183.000,26489.000,2311
2026-08-02T22:20:20.2450404Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1key_10000msg/String,19.000,22.000,32.100,787
2026-08-02T22:20:20.2452336Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1key_10000msg/String,14507.000,14808.000,20128.000,787
2026-08-02T22:20:20.2453637Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1key_10000msg/String,50.234,53.822,66.441,787
2026-08-02T22:20:20.2454919Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1key_10000msg/String,91.014,93.585,95.202,787
2026-08-02T22:20:20.2457140Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1key_10000msg/String,116.709,126.320,129.029,787
2026-08-02T22:20:20.2458925Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1key_10000msg/String,110.791,118.451,123.902,787
2026-08-02T22:20:20.2460468Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1key_10000msg/String,21840.000,22231.000,27381.000,787
2026-08-02T22:20:20.2461998Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1key_10000msg/u64,19.000,22.000,27.000,2391
2026-08-02T22:20:20.2463231Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1key_10000msg/u64,210.000,241.000,361.000,2391
2026-08-02T22:20:20.2464443Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1key_10000msg/u64,24.766,25.914,26.512,2391
2026-08-02T22:20:20.2465698Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1key_10000msg/u64,21.962,22.464,23.628,2391
2026-08-02T22:20:20.2467057Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1key_10000msg/u64,44.949,46.004,47.208,2391
2026-08-02T22:20:20.2468463Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1key_10000msg/u64,36.968,37.524,38.671,2391
2026-08-02T22:20:20.2469809Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1key_10000msg/u64,22612.000,23033.000,25417.000,2391
2026-08-02T22:20:20.2471106Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10000key_1msg/DropValue,20.000,24.100,36.100,707
2026-08-02T22:20:20.2472699Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10000key_1msg/DropValue,157.835,162.086,166.912,707
2026-08-02T22:20:20.2474091Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10000key_1msg/DropValue,29.660,30.355,31.791,707
2026-08-02T22:20:20.2475385Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10000key_1msg/DropValue,24.077,25.205,26.897,707
2026-08-02T22:20:20.2476779Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10000key_1msg/DropValue,64.189,67.033,68.673,707
2026-08-02T22:20:20.2478208Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10000key_1msg/DropValue,51.633,54.227,56.616,707
2026-08-02T22:20:20.2479582Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10000key_1msg/DropValue,150.937,152.661,160.539,707
2026-08-02T22:20:20.2480854Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10000key_1msg/String,20.000,24.000,36.000,527
2026-08-02T22:20:20.2482301Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10000key_1msg/String,168.015,173.421,177.608,527
2026-08-02T22:20:20.2483533Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10000key_1msg/String,52.492,54.708,57.374,527
2026-08-02T22:20:20.2484800Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10000key_1msg/String,82.955,85.706,87.736,527
2026-08-02T22:20:20.2486307Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10000key_1msg/String,126.384,133.854,137.033,527
2026-08-02T22:20:20.2487746Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10000key_1msg/String,116.779,123.570,129.760,527
2026-08-02T22:20:20.2489098Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10000key_1msg/String,150.793,153.027,174.274,527
2026-08-02T22:20:20.2490347Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10000key_1msg/u64,19.000,24.100,35.100,727
2026-08-02T22:20:20.2491837Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10000key_1msg/u64,155.024,159.513,166.128,727
2026-08-02T22:20:20.2493045Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10000key_1msg/u64,28.029,28.881,30.083,727
2026-08-02T22:20:20.2494288Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10000key_1msg/u64,23.729,24.214,25.855,727
2026-08-02T22:20:20.2495687Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10000key_1msg/u64,63.619,66.013,68.374,727
2026-08-02T22:20:20.2497095Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10000key_1msg/u64,52.575,54.329,56.105,727
2026-08-02T22:20:20.2498559Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10000key_1msg/u64,147.577,149.418,153.648,727
2026-08-02T22:20:20.2499839Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1000key_10msg/DropValue,19.100,23.000,28.100,1655
2026-08-02T22:20:20.2501106Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1000key_10msg/DropValue,158.074,163.143,174.144,1655
2026-08-02T22:20:20.2502620Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1000key_10msg/DropValue,28.145,28.682,29.398,1655
2026-08-02T22:20:20.2503915Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1000key_10msg/DropValue,22.960,23.616,24.783,1655
2026-08-02T22:20:20.2505316Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1000key_10msg/DropValue,54.294,56.307,57.728,1655
2026-08-02T22:20:20.2506750Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1000key_10msg/DropValue,42.629,44.015,45.532,1655
2026-08-02T22:20:20.2508120Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1000key_10msg/DropValue,173.302,176.799,190.584,1655
2026-08-02T22:20:20.2509403Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1000key_10msg/String,20.000,23.000,35.000,727
2026-08-02T22:20:20.2510805Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1000key_10msg/String,261.346,268.730,284.630,727
2026-08-02T22:20:20.2512297Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1000key_10msg/String,53.073,58.603,70.946,727
2026-08-02T22:20:20.2513565Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1000key_10msg/String,81.878,84.166,86.142,727
2026-08-02T22:20:20.2514945Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1000key_10msg/String,117.711,123.559,127.602,727
2026-08-02T22:20:20.2517622Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1000key_10msg/String,109.167,114.284,118.068,727
2026-08-02T22:20:20.2519060Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1000key_10msg/String,172.831,176.729,195.153,727
2026-08-02T22:20:20.2520350Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1000key_10msg/u64,19.000,23.000,29.000,1695
2026-08-02T22:20:20.2521872Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1000key_10msg/u64,151.482,156.802,169.565,1695
2026-08-02T22:20:20.2523137Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1000key_10msg/u64,25.608,26.444,27.278,1695
2026-08-02T22:20:20.2524576Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1000key_10msg/u64,22.481,23.194,24.447,1695
2026-08-02T22:20:20.2525930Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1000key_10msg/u64,52.560,54.084,55.494,1695
2026-08-02T22:20:20.2527323Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1000key_10msg/u64,42.032,43.413,44.904,1695
2026-08-02T22:20:20.2528651Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1000key_10msg/u64,170.176,173.674,184.884,1695
2026-08-02T22:20:20.2529916Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/100key_100msg/DropValue,20.000,23.000,31.100,2191
2026-08-02T22:20:20.2531187Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/100key_100msg/DropValue,223.520,239.050,334.020,2191
2026-08-02T22:20:20.2532696Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/100key_100msg/DropValue,27.635,28.227,29.366,2191
2026-08-02T22:20:20.2533993Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/100key_100msg/DropValue,22.642,23.502,25.217,2191
2026-08-02T22:20:20.2535430Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/100key_100msg/DropValue,49.585,51.239,52.889,2191
2026-08-02T22:20:20.2537013Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/100key_100msg/DropValue,38.752,40.381,42.624,2191
2026-08-02T22:20:20.2538393Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/100key_100msg/DropValue,383.610,395.930,495.120,2191
2026-08-02T22:20:20.2539678Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/100key_100msg/String,19.000,23.000,30.000,787
2026-08-02T22:20:20.2541818Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/100key_100msg/String,1294.710,1333.880,1438.480,787
2026-08-02T22:20:20.2543075Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/100key_100msg/String,51.167,55.285,68.149,787
2026-08-02T22:20:20.2544342Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/100key_100msg/String,83.140,87.994,92.131,787
2026-08-02T22:20:20.2545732Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/100key_100msg/String,116.602,122.230,145.670,787
2026-08-02T22:20:20.2547175Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/100key_100msg/String,108.990,114.233,120.131,787
2026-08-02T22:20:20.2548574Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/100key_100msg/String,391.630,406.150,502.330,787
2026-08-02T22:20:20.2550005Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/100key_100msg/u64,19.000,22.000,28.100,2231
2026-08-02T22:20:20.2551454Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/100key_100msg/u64,142.260,156.900,168.220,2231
2026-08-02T22:20:20.2552723Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/100key_100msg/u64,25.071,26.107,27.003,2231
2026-08-02T22:20:20.2553967Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/100key_100msg/u64,21.944,22.863,24.237,2231
2026-08-02T22:20:20.2555315Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/100key_100msg/u64,49.703,51.562,52.861,2231
2026-08-02T22:20:20.2556708Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/100key_100msg/u64,40.050,41.518,42.995,2231
2026-08-02T22:20:20.2558031Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/100key_100msg/u64,369.590,383.310,477.990,2231
2026-08-02T22:20:20.2559294Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10key_1000msg/DropValue,19.000,22.000,27.100,2311
2026-08-02T22:20:20.2560570Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10key_1000msg/DropValue,649.200,716.400,923.700,2311
2026-08-02T22:20:20.2562261Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10key_1000msg/DropValue,27.432,27.986,28.609,2311
2026-08-02T22:20:20.2563561Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10key_1000msg/DropValue,21.559,23.143,24.669,2311
2026-08-02T22:20:20.2564956Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10key_1000msg/DropValue,46.538,48.348,69.576,2311
2026-08-02T22:20:20.2566407Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10key_1000msg/DropValue,37.018,38.019,41.245,2311
2026-08-02T22:20:20.2567795Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10key_1000msg/DropValue,1634.000,1696.200,1769.300,2311
2026-08-02T22:20:20.2569096Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10key_1000msg/String,19.000,22.000,31.100,787
2026-08-02T22:20:20.2570360Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10key_1000msg/String,11116.600,11598.600,12738.700,787
2026-08-02T22:20:20.2571789Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10key_1000msg/String,52.165,62.803,71.300,787
2026-08-02T22:20:20.2573089Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10key_1000msg/String,81.059,84.553,88.293,787
2026-08-02T22:20:20.2574467Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10key_1000msg/String,111.274,119.103,141.509,787
2026-08-02T22:20:20.2576021Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10key_1000msg/String,105.373,111.113,117.414,787
2026-08-02T22:20:20.2577390Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10key_1000msg/String,1607.000,1669.100,1979.700,787
2026-08-02T22:20:20.2578645Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10key_1000msg/u64,19.000,22.000,28.000,2331
2026-08-02T22:20:20.2579899Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10key_1000msg/u64,136.300,158.300,197.400,2331
2026-08-02T22:20:20.2581110Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10key_1000msg/u64,24.849,25.979,26.937,2331
2026-08-02T22:20:20.2582560Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10key_1000msg/u64,21.422,22.448,25.013,2331
2026-08-02T22:20:20.2583897Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10key_1000msg/u64,46.026,47.396,49.091,2331
2026-08-02T22:20:20.2585286Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10key_1000msg/u64,37.880,38.564,40.437,2331
2026-08-02T22:20:20.2586748Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10key_1000msg/u64,1620.000,1659.100,1790.300,2331
2026-08-02T22:20:20.2588026Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1key_10000msg/DropValue,19.000,24.000,37.100,2311
2026-08-02T22:20:20.2589322Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1key_10000msg/DropValue,9628.000,11271.000,15750.000,2311
2026-08-02T22:20:20.2590603Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1key_10000msg/DropValue,27.408,28.020,28.706,2311
2026-08-02T22:20:20.2592168Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1key_10000msg/DropValue,21.624,22.681,23.973,2311
2026-08-02T22:20:20.2593573Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1key_10000msg/DropValue,46.252,47.001,48.151,2311
2026-08-02T22:20:20.2595014Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1key_10000msg/DropValue,36.738,37.174,39.038,2311
2026-08-02T22:20:20.2596421Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1key_10000msg/DropValue,22732.000,23263.000,27651.000,2311
2026-08-02T22:20:20.2597723Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1key_10000msg/String,20.000,26.100,38.000,807
2026-08-02T22:20:20.2598990Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1key_10000msg/String,116527.000,125393.000,136454.000,807
2026-08-02T22:20:20.2600403Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1key_10000msg/String,50.285,54.575,68.171,807
2026-08-02T22:20:20.2601893Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1key_10000msg/String,81.007,83.038,85.322,807
2026-08-02T22:20:20.2603285Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1key_10000msg/String,109.209,115.769,118.069,807
2026-08-02T22:20:20.2604707Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1key_10000msg/String,103.579,109.886,112.959,807
2026-08-02T22:20:20.2606079Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1key_10000msg/String,21680.000,22221.000,27481.000,807
2026-08-02T22:20:20.2607345Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1key_10000msg/u64,19.000,22.000,28.000,2371
2026-08-02T22:20:20.2608568Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1key_10000msg/u64,210.000,250.000,451.000,2371
2026-08-02T22:20:20.2609769Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1key_10000msg/u64,24.741,25.981,26.911,2371
2026-08-02T22:20:20.2611134Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1key_10000msg/u64,21.684,22.730,23.907,2371
2026-08-02T22:20:20.2612676Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1key_10000msg/u64,44.975,46.140,47.524,2371
2026-08-02T22:20:20.2614073Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1key_10000msg/u64,37.093,37.720,39.160,2371
2026-08-02T22:20:20.2615412Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1key_10000msg/u64,22743.000,23163.000,29425.000,2371
2026-08-02T22:20:20.2616698Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10000key_1msg/DropValue,21.100,30.100,45.100,171
2026-08-02T22:20:20.2617983Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10000key_1msg/DropValue,38.564,39.168,39.805,171
2026-08-02T22:20:20.2619242Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10000key_1msg/DropValue,28.588,28.811,29.230,171
2026-08-02T22:20:20.2620551Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10000key_1msg/DropValue,24.248,25.312,26.608,171
2026-08-02T22:20:20.2622175Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10000key_1msg/DropValue,63.419,65.943,70.302,171
2026-08-02T22:20:20.2623634Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10000key_1msg/DropValue,51.028,52.949,55.972,171
2026-08-02T22:20:20.2625125Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10000key_1msg/DropValue,35.307,35.523,36.096,171
2026-08-02T22:20:20.2626405Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10000key_1msg/String,23.000,35.100,53.100,131
2026-08-02T22:20:20.2627653Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10000key_1msg/String,38.976,39.320,39.919,131
2026-08-02T22:20:20.2628879Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10000key_1msg/String,44.760,45.796,46.090,131
2026-08-02T22:20:20.2630155Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10000key_1msg/String,83.737,85.561,87.739,131
2026-08-02T22:20:20.2631825Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10000key_1msg/String,126.426,131.486,142.324,131
2026-08-02T22:20:20.2633272Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10000key_1msg/String,118.111,125.297,133.243,131
2026-08-02T22:20:20.2634620Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10000key_1msg/String,35.221,35.541,36.115,131
2026-08-02T22:20:20.2636003Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10000key_1msg/u64,23.100,33.100,48.100,171
2026-08-02T22:20:20.2637215Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10000key_1msg/u64,38.044,39.676,40.564,171
2026-08-02T22:20:20.2638417Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10000key_1msg/u64,26.729,27.607,27.904,171
2026-08-02T22:20:20.2639655Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10000key_1msg/u64,23.863,24.287,25.546,171
2026-08-02T22:20:20.2640991Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10000key_1msg/u64,62.407,64.444,67.885,171
2026-08-02T22:20:20.2642528Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10000key_1msg/u64,51.201,53.497,57.273,171
2026-08-02T22:20:20.2643851Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10000key_1msg/u64,35.167,45.341,47.321,171
2026-08-02T22:20:20.2645115Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1000key_10msg/DropValue,20.000,25.000,34.000,343
2026-08-02T22:20:20.2646382Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1000key_10msg/DropValue,38.597,39.352,40.255,343
2026-08-02T22:20:20.2647637Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1000key_10msg/DropValue,27.154,27.362,27.588,343
2026-08-02T22:20:20.2649054Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1000key_10msg/DropValue,23.212,23.620,24.686,343
2026-08-02T22:20:20.2650449Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1000key_10msg/DropValue,55.888,57.680,59.634,343
2026-08-02T22:20:20.2651998Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1000key_10msg/DropValue,43.718,45.252,47.720,343
2026-08-02T22:20:20.2653363Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1000key_10msg/DropValue,36.542,37.424,38.423,343
2026-08-02T22:20:20.2654638Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1000key_10msg/String,20.000,25.100,35.100,191
2026-08-02T22:20:20.2655877Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1000key_10msg/String,44.430,45.246,46.325,191
2026-08-02T22:20:20.2657100Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1000key_10msg/String,43.859,44.752,45.082,191
2026-08-02T22:20:20.2658569Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1000key_10msg/String,82.702,83.964,85.329,191
2026-08-02T22:20:20.2659945Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1000key_10msg/String,121.115,126.234,132.995,191
2026-08-02T22:20:20.2661639Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1000key_10msg/String,112.991,117.069,119.645,191
2026-08-02T22:20:20.2662986Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1000key_10msg/String,36.953,37.713,39.057,191
2026-08-02T22:20:20.2664240Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1000key_10msg/u64,19.000,24.100,33.000,363
2026-08-02T22:20:20.2665439Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1000key_10msg/u64,37.383,38.172,39.189,363
2026-08-02T22:20:20.2666645Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1000key_10msg/u64,24.945,25.503,25.695,363
2026-08-02T22:20:20.2667887Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1000key_10msg/u64,22.882,23.273,25.747,363
2026-08-02T22:20:20.2669235Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1000key_10msg/u64,54.082,55.667,57.441,363
2026-08-02T22:20:20.2670613Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1000key_10msg/u64,43.376,44.599,46.872,363
2026-08-02T22:20:20.2672093Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1000key_10msg/u64,36.556,37.407,38.281,363
2026-08-02T22:20:20.2673534Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/100key_100msg/DropValue,19.000,24.100,32.100,383
2026-08-02T22:20:20.2674814Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/100key_100msg/DropValue,42.053,42.992,48.947,383
2026-08-02T22:20:20.2676071Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/100key_100msg/DropValue,26.822,26.962,27.205,383
2026-08-02T22:20:20.2677372Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/100key_100msg/DropValue,22.632,23.457,24.583,383
2026-08-02T22:20:20.2678772Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/100key_100msg/DropValue,51.506,52.836,54.196,383
2026-08-02T22:20:20.2680220Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/100key_100msg/DropValue,40.688,41.753,43.696,383
2026-08-02T22:20:20.2681737Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/100key_100msg/DropValue,51.759,53.093,59.141,383
2026-08-02T22:20:20.2683013Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/100key_100msg/String,20.000,26.100,35.100,211
2026-08-02T22:20:20.2684258Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/100key_100msg/String,41.866,42.867,49.448,211
2026-08-02T22:20:20.2685611Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/100key_100msg/String,43.248,44.107,44.724,211
2026-08-02T22:20:20.2686874Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/100key_100msg/String,83.853,87.861,92.311,211
2026-08-02T22:20:20.2688252Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/100key_100msg/String,118.925,122.540,125.261,211
2026-08-02T22:20:20.2689697Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/100key_100msg/String,110.909,114.035,116.728,211
2026-08-02T22:20:20.2691053Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/100key_100msg/String,53.399,124.118,130.925,211
2026-08-02T22:20:20.2692431Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/100key_100msg/u64,19.000,24.000,34.000,383
2026-08-02T22:20:20.2693639Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/100key_100msg/u64,36.343,37.338,42.936,383
2026-08-02T22:20:20.2694845Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/100key_100msg/u64,24.612,25.163,25.431,383
2026-08-02T22:20:20.2696085Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/100key_100msg/u64,22.332,22.987,24.865,383
2026-08-02T22:20:20.2697546Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/100key_100msg/u64,51.298,52.688,54.112,383
2026-08-02T22:20:20.2698937Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/100key_100msg/u64,40.902,42.286,43.837,383
2026-08-02T22:20:20.2700255Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/100key_100msg/u64,51.333,52.554,59.524,383
2026-08-02T22:20:20.2701622Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10key_1000msg/DropValue,19.000,24.100,34.100,363
2026-08-02T22:20:20.2702883Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10key_1000msg/DropValue,74.644,77.394,97.181,363
2026-08-02T22:20:20.2704145Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10key_1000msg/DropValue,27.289,27.498,27.969,363
2026-08-02T22:20:20.2705445Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10key_1000msg/DropValue,21.883,22.728,25.460,363
2026-08-02T22:20:20.2706841Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10key_1000msg/DropValue,46.605,47.759,49.339,363
2026-08-02T22:20:20.2708280Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10key_1000msg/DropValue,36.988,37.644,39.688,363
2026-08-02T22:20:20.2709782Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10key_1000msg/DropValue,129.431,132.619,169.562,363
2026-08-02T22:20:20.2711080Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10key_1000msg/String,20.000,26.100,39.100,211
2026-08-02T22:20:20.2712445Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10key_1000msg/String,727.919,764.113,833.612,211
2026-08-02T22:20:20.2713681Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10key_1000msg/String,43.287,44.187,46.016,211
2026-08-02T22:20:20.2714951Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10key_1000msg/String,81.077,84.414,88.519,211
2026-08-02T22:20:20.2716341Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10key_1000msg/String,111.746,114.588,120.551,211
2026-08-02T22:20:20.2717771Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10key_1000msg/String,106.256,111.642,115.850,211
2026-08-02T22:20:20.2719125Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10key_1000msg/String,128.363,140.831,151.344,211
2026-08-02T22:20:20.2720377Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10key_1000msg/u64,19.000,24.000,33.000,383
2026-08-02T22:20:20.2721714Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10key_1000msg/u64,36.881,38.200,44.019,383
2026-08-02T22:20:20.2723031Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10key_1000msg/u64,25.112,26.098,26.276,383
2026-08-02T22:20:20.2724281Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10key_1000msg/u64,21.743,22.434,23.407,383
2026-08-02T22:20:20.2725629Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10key_1000msg/u64,45.142,46.254,47.217,383
2026-08-02T22:20:20.2727021Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10key_1000msg/u64,37.179,37.694,39.080,383
2026-08-02T22:20:20.2728348Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10key_1000msg/u64,128.431,131.744,138.881,383
2026-08-02T22:20:20.2729618Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1key_10000msg/DropValue,19.000,24.100,37.100,383
2026-08-02T22:20:20.2730908Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1key_10000msg/DropValue,656.188,747.625,1175.938,383
2026-08-02T22:20:20.2732314Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1key_10000msg/DropValue,27.463,27.647,27.855,383
2026-08-02T22:20:20.2733732Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1key_10000msg/DropValue,21.889,22.705,23.968,383
2026-08-02T22:20:20.2735127Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1key_10000msg/DropValue,46.316,47.459,48.722,383
2026-08-02T22:20:20.2736570Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1key_10000msg/DropValue,36.812,37.266,38.648,383
2026-08-02T22:20:20.2737958Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1key_10000msg/DropValue,1462.062,1498.438,1628.062,383
2026-08-02T22:20:20.2739256Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1key_10000msg/String,20.000,25.000,33.100,211
2026-08-02T22:20:20.2740513Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1key_10000msg/String,7377.500,7872.188,8536.562,211
2026-08-02T22:20:20.2741883Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1key_10000msg/String,43.658,44.682,45.453,211
2026-08-02T22:20:20.2743153Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1key_10000msg/String,81.056,82.875,85.680,211
2026-08-02T22:20:20.2744543Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1key_10000msg/String,110.590,112.445,118.659,211
2026-08-02T22:20:20.2745998Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1key_10000msg/String,104.590,111.128,113.482,211
2026-08-02T22:20:20.2747534Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1key_10000msg/String,1403.250,1481.500,1567.938,211
2026-08-02T22:20:20.2748812Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1key_10000msg/u64,19.000,23.100,31.100,383
2026-08-02T22:20:20.2750022Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1key_10000msg/u64,43.812,48.812,70.750,383
2026-08-02T22:20:20.2751337Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1key_10000msg/u64,25.206,26.260,26.454,383
2026-08-02T22:20:20.2752592Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1key_10000msg/u64,21.559,22.566,23.809,383
2026-08-02T22:20:20.2753936Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1key_10000msg/u64,44.902,46.106,47.457,383
2026-08-02T22:20:20.2755324Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1key_10000msg/u64,37.056,38.265,39.555,383
2026-08-02T22:20:20.2756652Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1key_10000msg/u64,1462.062,1489.062,1643.625,383
2026-08-02T22:20:20.2758057Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10000key_1msg/DropValue,21.000,27.000,36.100,423
2026-08-02T22:20:20.2759326Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10000key_1msg/DropValue,62.525,63.878,65.490,423
2026-08-02T22:20:20.2760570Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10000key_1msg/DropValue,28.341,28.642,29.280,423
2026-08-02T22:20:20.2761980Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10000key_1msg/DropValue,24.230,24.589,25.876,423
2026-08-02T22:20:20.2763368Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10000key_1msg/DropValue,66.640,68.844,72.246,423
2026-08-02T22:20:20.2764811Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10000key_1msg/DropValue,53.098,56.144,59.454,423
2026-08-02T22:20:20.2766172Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10000key_1msg/DropValue,58.502,58.949,60.385,423
2026-08-02T22:20:20.2767447Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10000key_1msg/String,22.000,27.100,43.100,303
2026-08-02T22:20:20.2768676Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10000key_1msg/String,66.711,67.873,69.479,303
2026-08-02T22:20:20.2769894Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10000key_1msg/String,47.025,48.746,50.507,303
2026-08-02T22:20:20.2771404Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10000key_1msg/String,83.672,85.657,87.842,303
2026-08-02T22:20:20.2772895Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10000key_1msg/String,128.529,136.489,139.437,303
2026-08-02T22:20:20.2774328Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10000key_1msg/String,119.869,125.723,130.236,303
2026-08-02T22:20:20.2775677Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10000key_1msg/String,58.583,59.269,62.417,303
2026-08-02T22:20:20.2776919Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10000key_1msg/u64,19.000,25.000,35.100,423
2026-08-02T22:20:20.2778134Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10000key_1msg/u64,63.122,64.117,65.569,423
2026-08-02T22:20:20.2779319Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10000key_1msg/u64,26.598,27.568,28.170,423
2026-08-02T22:20:20.2780546Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10000key_1msg/u64,23.933,24.210,25.449,423
2026-08-02T22:20:20.2781988Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10000key_1msg/u64,66.048,68.568,72.196,423
2026-08-02T22:20:20.2783505Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10000key_1msg/u64,52.075,56.133,59.413,423
2026-08-02T22:20:20.2784822Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10000key_1msg/u64,57.751,58.235,59.429,423
2026-08-02T22:20:20.2786081Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1000key_10msg/DropValue,21.000,26.000,33.100,887
2026-08-02T22:20:20.2787338Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1000key_10msg/DropValue,63.679,64.944,69.457,887
2026-08-02T22:20:20.2788597Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1000key_10msg/DropValue,27.549,27.841,28.336,887
2026-08-02T22:20:20.2789905Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1000key_10msg/DropValue,23.287,23.690,24.900,887
2026-08-02T22:20:20.2791408Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1000key_10msg/DropValue,56.032,57.918,60.236,887
2026-08-02T22:20:20.2792843Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1000key_10msg/DropValue,43.879,45.464,47.931,887
2026-08-02T22:20:20.2794206Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1000key_10msg/DropValue,64.365,65.542,69.757,887
2026-08-02T22:20:20.2795594Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1000key_10msg/String,22.000,25.100,35.000,527
2026-08-02T22:20:20.2796830Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1000key_10msg/String,89.665,91.656,95.570,527
2026-08-02T22:20:20.2798049Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1000key_10msg/String,46.445,47.215,47.992,527
2026-08-02T22:20:20.2799314Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1000key_10msg/String,81.915,84.106,85.876,527
2026-08-02T22:20:20.2800697Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1000key_10msg/String,118.771,125.695,129.500,527
2026-08-02T22:20:20.2802237Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1000key_10msg/String,111.247,116.365,120.951,527
2026-08-02T22:20:20.2803577Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1000key_10msg/String,64.736,66.031,69.770,527
2026-08-02T22:20:20.2804813Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1000key_10msg/u64,19.000,25.000,31.100,1035
2026-08-02T22:20:20.2806017Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1000key_10msg/u64,62.579,63.957,67.969,1035
2026-08-02T22:20:20.2807340Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1000key_10msg/u64,25.305,25.854,26.194,1035
2026-08-02T22:20:20.2808583Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1000key_10msg/u64,23.006,23.512,24.756,1035
2026-08-02T22:20:20.2809920Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1000key_10msg/u64,54.248,56.142,57.965,1035
2026-08-02T22:20:20.2811414Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1000key_10msg/u64,43.489,44.927,47.196,1035
2026-08-02T22:20:20.2812735Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1000key_10msg/u64,63.733,64.993,68.658,1035
2026-08-02T22:20:20.2813998Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/100key_100msg/DropValue,20.000,23.000,30.100,1135
2026-08-02T22:20:20.2815257Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/100key_100msg/DropValue,77.895,80.575,98.632,1135
2026-08-02T22:20:20.2816509Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/100key_100msg/DropValue,27.108,27.307,27.833,1135
2026-08-02T22:20:20.2817801Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/100key_100msg/DropValue,22.814,24.048,25.413,1135
2026-08-02T22:20:20.2819205Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/100key_100msg/DropValue,50.181,52.644,54.947,1135
2026-08-02T22:20:20.2820764Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/100key_100msg/DropValue,39.544,40.890,42.578,1135
2026-08-02T22:20:20.2822257Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/100key_100msg/DropValue,120.700,124.180,147.800,1135
2026-08-02T22:20:20.2823540Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/100key_100msg/String,20.000,23.100,31.000,547
2026-08-02T22:20:20.2824790Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/100key_100msg/String,347.348,364.605,397.340,547
2026-08-02T22:20:20.2826024Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/100key_100msg/String,45.667,46.430,48.762,547
2026-08-02T22:20:20.2827294Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/100key_100msg/String,83.981,88.063,91.904,547
2026-08-02T22:20:20.2828670Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/100key_100msg/String,116.737,121.509,125.028,547
2026-08-02T22:20:20.2830108Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/100key_100msg/String,109.463,113.317,116.425,547
2026-08-02T22:20:20.2831692Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/100key_100msg/String,124.382,130.545,155.540,547
2026-08-02T22:20:20.2832950Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/100key_100msg/u64,19.000,23.000,30.100,1175
2026-08-02T22:20:20.2834155Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/100key_100msg/u64,56.983,60.640,65.120,1175
2026-08-02T22:20:20.2835352Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/100key_100msg/u64,24.742,25.459,25.807,1175
2026-08-02T22:20:20.2836595Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/100key_100msg/u64,22.449,22.995,24.254,1175
2026-08-02T22:20:20.2837936Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/100key_100msg/u64,50.091,51.623,53.205,1175
2026-08-02T22:20:20.2839321Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/100key_100msg/u64,39.300,40.990,42.611,1175
2026-08-02T22:20:20.2840638Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/100key_100msg/u64,116.945,121.375,144.593,1175
2026-08-02T22:20:20.2842016Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10key_1000msg/DropValue,19.100,23.000,30.100,1175
2026-08-02T22:20:20.2843291Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10key_1000msg/DropValue,184.850,202.875,293.550,1175
2026-08-02T22:20:20.2844697Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10key_1000msg/DropValue,27.371,27.580,28.096,1175
2026-08-02T22:20:20.2845994Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10key_1000msg/DropValue,22.328,23.600,25.725,1175
2026-08-02T22:20:20.2847388Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10key_1000msg/DropValue,46.565,48.130,49.967,1175
2026-08-02T22:20:20.2848827Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10key_1000msg/DropValue,37.055,37.581,39.613,1175
2026-08-02T22:20:20.2850214Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10key_1000msg/DropValue,431.300,439.800,464.600,1175
2026-08-02T22:20:20.2851608Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10key_1000msg/String,20.000,23.000,30.000,547
2026-08-02T22:20:20.2852857Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10key_1000msg/String,2831.525,2963.000,3330.500,547
2026-08-02T22:20:20.2854102Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10key_1000msg/String,45.189,46.418,50.060,547
2026-08-02T22:20:20.2855491Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10key_1000msg/String,81.400,84.737,88.182,547
2026-08-02T22:20:20.2856865Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10key_1000msg/String,112.064,117.416,120.861,547
2026-08-02T22:20:20.2858298Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10key_1000msg/String,106.608,111.630,116.340,547
2026-08-02T22:20:20.2859654Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10key_1000msg/String,430.050,475.150,537.750,547
2026-08-02T22:20:20.2860908Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10key_1000msg/u64,19.000,22.000,29.000,1215
2026-08-02T22:20:20.2862232Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10key_1000msg/u64,57.850,62.850,77.150,1215
2026-08-02T22:20:20.2863426Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10key_1000msg/u64,25.134,26.116,26.464,1215
2026-08-02T22:20:20.2864663Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10key_1000msg/u64,21.819,22.476,23.573,1215
2026-08-02T22:20:20.2866008Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10key_1000msg/u64,45.198,46.270,47.720,1215
2026-08-02T22:20:20.2867389Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10key_1000msg/u64,37.269,37.750,39.273,1215
2026-08-02T22:20:20.2869164Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10key_1000msg/u64,430.800,444.075,494.925,1215
2026-08-02T22:20:20.2870567Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1key_10000msg/DropValue,19.000,23.100,36.000,1175
2026-08-02T22:20:20.2872398Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1key_10000msg/DropValue,2306.750,2993.000,4613.750,1175
2026-08-02T22:20:20.2873807Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1key_10000msg/DropValue,27.434,27.656,28.084,1175
2026-08-02T22:20:20.2875116Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1key_10000msg/DropValue,22.199,22.704,23.817,1175
2026-08-02T22:20:20.2876506Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1key_10000msg/DropValue,46.301,47.327,48.328,1175
2026-08-02T22:20:20.2877948Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1key_10000msg/DropValue,36.778,37.202,38.377,1175
2026-08-02T22:20:20.2879336Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1key_10000msg/DropValue,5713.250,5805.750,7471.500,1175
2026-08-02T22:20:20.2880751Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1key_10000msg/String,22.000,25.000,40.000,547
2026-08-02T22:20:20.2882128Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1key_10000msg/String,29815.500,31431.000,34654.500,547
2026-08-02T22:20:20.2883380Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1key_10000msg/String,45.309,46.423,49.893,547
2026-08-02T22:20:20.2884652Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1key_10000msg/String,81.318,83.046,85.016,547
2026-08-02T22:20:20.2886029Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1key_10000msg/String,109.753,116.471,119.001,547
2026-08-02T22:20:20.2887500Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1key_10000msg/String,104.490,110.860,113.471,547
2026-08-02T22:20:20.2888860Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1key_10000msg/String,5482.750,5861.000,7060.750,547
2026-08-02T22:20:20.2890122Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1key_10000msg/u64,19.000,22.000,28.000,1215
2026-08-02T22:20:20.2891453Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1key_10000msg/u64,82.750,92.750,155.500,1215
2026-08-02T22:20:20.2892671Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1key_10000msg/u64,25.144,26.214,26.501,1215
2026-08-02T22:20:20.2894028Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1key_10000msg/u64,21.616,22.468,23.595,1215
2026-08-02T22:20:20.2895373Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1key_10000msg/u64,44.921,45.629,46.726,1215
2026-08-02T22:20:20.2896754Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1key_10000msg/u64,36.997,37.455,38.684,1215
2026-08-02T22:20:20.2898086Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1key_10000msg/u64,5683.000,5755.750,5976.250,1215
2026-08-02T22:20:20.2928603Z ##[group]Run cargo bench --locked --features bench-variants --bench key_stream -- --noplot \
2026-08-02T22:20:20.2929335Z [36;1mcargo bench --locked --features bench-variants --bench key_stream -- --noplot \[0m
2026-08-02T22:20:20.2929781Z [36;1m  | tee "$OUT/on.raw.txt"[0m
2026-08-02T22:20:20.2973227Z shell: /usr/bin/bash -e {0}
2026-08-02T22:20:20.2973476Z env:
2026-08-02T22:20:20.2973674Z   CARGO_HOME: /home/runner/.cargo
2026-08-02T22:20:20.2973948Z   CARGO_INCREMENTAL: 0
2026-08-02T22:20:20.2974178Z   CARGO_TERM_COLOR: always
2026-08-02T22:20:20.2974435Z   OUT: /home/runner/work/_temp/bench
2026-08-02T22:20:20.2974696Z ##[endgroup]
2026-08-02T22:20:20.3663084Z [1m[92m   Compiling[0m key-stream v0.10.0 (/home/runner/work/key-stream/key-stream)
2026-08-02T22:20:24.0115369Z [1m[92m    Finished[0m `bench` profile [optimized] target(s) in 3.69s
2026-08-02T22:20:24.0165822Z [1m[92m     Running[0m benches/key_stream.rs (target/release/deps/key_stream-bc830f5ae3d4d2e8)
2026-08-02T22:20:24.0179112Z Gnuplot not found, using plotters backend
2026-08-02T22:20:24.0813827Z Benchmarking full_cycle/1x1/1key_10000msg/u64
2026-08-02T22:20:24.0814599Z Benchmarking full_cycle/1x1/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:20:24.8280190Z Benchmarking full_cycle/1x1/1key_10000msg/u64: Collecting 20 samples in estimated 3.0448 s (1040 iterations)
2026-08-02T22:20:27.8896251Z Benchmarking full_cycle/1x1/1key_10000msg/u64: Analyzing
2026-08-02T22:20:27.9480002Z full_cycle/1x1/1key_10000msg/u64
2026-08-02T22:20:27.9480907Z                         time:   [2.9345 ms 2.9423 ms 2.9493 ms]
2026-08-02T22:20:27.9482156Z                         thrpt:  [30.516 Melem/s 30.589 Melem/s 30.670 Melem/s]
2026-08-02T22:20:27.9495062Z                  change:
2026-08-02T22:20:27.9495619Z                         time:   [+80.017% +81.342% +82.326%] (p = 0.00 < 0.05)
2026-08-02T22:20:27.9496357Z                         thrpt:  [−45.153% −44.856% −44.450%]
2026-08-02T22:20:27.9496733Z                         Performance has regressed.
2026-08-02T22:20:27.9497109Z Benchmarking full_cycle/1x1/1key_10000msg/String
2026-08-02T22:20:27.9497537Z Benchmarking full_cycle/1x1/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:20:28.9084532Z Benchmarking full_cycle/1x1/1key_10000msg/String: Collecting 20 samples in estimated 3.0218 s (400 iterations)
2026-08-02T22:20:31.9899341Z Benchmarking full_cycle/1x1/1key_10000msg/String: Analyzing
2026-08-02T22:20:32.0466491Z full_cycle/1x1/1key_10000msg/String
2026-08-02T22:20:32.0467053Z                         time:   [7.6565 ms 7.7016 ms 7.7411 ms]
2026-08-02T22:20:32.0467689Z                         thrpt:  [11.627 Melem/s 11.686 Melem/s 11.755 Melem/s]
2026-08-02T22:20:32.0468273Z                  change:
2026-08-02T22:20:32.0468759Z                         time:   [+75.687% +77.624% +79.195%] (p = 0.00 < 0.05)
2026-08-02T22:20:32.0469665Z                         thrpt:  [−44.195% −43.701% −43.081%]
2026-08-02T22:20:32.0470303Z                         Performance has regressed.
2026-08-02T22:20:32.0470740Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:20:32.0471048Z   1 (5.00%) low mild
2026-08-02T22:20:32.0476064Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue
2026-08-02T22:20:32.0476649Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:20:32.8324361Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.0156 s (980 iterations)
2026-08-02T22:20:35.8397392Z Benchmarking full_cycle/1x1/1key_10000msg/DropValue: Analyzing
2026-08-02T22:20:35.8958232Z full_cycle/1x1/1key_10000msg/DropValue
2026-08-02T22:20:35.8958800Z                         time:   [3.0512 ms 3.0669 ms 3.0832 ms]
2026-08-02T22:20:35.8959409Z                         thrpt:  [29.191 Melem/s 29.346 Melem/s 29.497 Melem/s]
2026-08-02T22:20:35.8959966Z                  change:
2026-08-02T22:20:35.8960420Z                         time:   [+82.157% +83.167% +84.293%] (p = 0.00 < 0.05)
2026-08-02T22:20:35.8961836Z                         thrpt:  [−45.738% −45.405% −45.102%]
2026-08-02T22:20:35.8962222Z                         Performance has regressed.
2026-08-02T22:20:35.8965850Z Benchmarking full_cycle/1x1/10key_1000msg/u64
2026-08-02T22:20:35.8966391Z Benchmarking full_cycle/1x1/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:20:36.6537417Z Benchmarking full_cycle/1x1/10key_1000msg/u64: Collecting 20 samples in estimated 3.0278 s (1020 iterations)
2026-08-02T22:20:39.6896701Z Benchmarking full_cycle/1x1/10key_1000msg/u64: Analyzing
2026-08-02T22:20:39.7465818Z full_cycle/1x1/10key_1000msg/u64
2026-08-02T22:20:39.7466375Z                         time:   [2.9682 ms 2.9748 ms 2.9807 ms]
2026-08-02T22:20:39.7467009Z                         thrpt:  [30.201 Melem/s 30.261 Melem/s 30.328 Melem/s]
2026-08-02T22:20:39.7467601Z                  change:
2026-08-02T22:20:39.7468070Z                         time:   [+81.896% +82.623% +83.292%] (p = 0.00 < 0.05)
2026-08-02T22:20:39.7469331Z                         thrpt:  [−45.442% −45.242% −45.023%]
2026-08-02T22:20:39.7469910Z                         Performance has regressed.
2026-08-02T22:20:39.7470272Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:20:39.7470570Z   2 (10.00%) low mild
2026-08-02T22:20:39.7473170Z Benchmarking full_cycle/1x1/10key_1000msg/String
2026-08-02T22:20:39.7473734Z Benchmarking full_cycle/1x1/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:20:40.7441540Z Benchmarking full_cycle/1x1/10key_1000msg/String: Collecting 20 samples in estimated 3.1388 s (400 iterations)
2026-08-02T22:20:43.9065412Z Benchmarking full_cycle/1x1/10key_1000msg/String: Analyzing
2026-08-02T22:20:43.9630775Z full_cycle/1x1/10key_1000msg/String
2026-08-02T22:20:43.9631536Z                         time:   [7.8528 ms 7.9040 ms 7.9545 ms]
2026-08-02T22:20:43.9632170Z                         thrpt:  [11.317 Melem/s 11.389 Melem/s 11.463 Melem/s]
2026-08-02T22:20:43.9632782Z                  change:
2026-08-02T22:20:43.9633280Z                         time:   [+68.863% +71.269% +73.784%] (p = 0.00 < 0.05)
2026-08-02T22:20:43.9634291Z                         thrpt:  [−42.457% −41.612% −40.780%]
2026-08-02T22:20:43.9634860Z                         Performance has regressed.
2026-08-02T22:20:43.9637324Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue
2026-08-02T22:20:43.9637959Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:20:44.7403394Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0447 s (1000 iterations)
2026-08-02T22:20:47.7795625Z Benchmarking full_cycle/1x1/10key_1000msg/DropValue: Analyzing
2026-08-02T22:20:47.8372192Z full_cycle/1x1/10key_1000msg/DropValue
2026-08-02T22:20:47.8372780Z                         time:   [3.0310 ms 3.0377 ms 3.0436 ms]
2026-08-02T22:20:47.8373402Z                         thrpt:  [29.577 Melem/s 29.634 Melem/s 29.700 Melem/s]
2026-08-02T22:20:47.8374016Z                  change:
2026-08-02T22:20:47.8374506Z                         time:   [+65.726% +70.476% +75.392%] (p = 0.00 < 0.05)
2026-08-02T22:20:47.8375426Z                         thrpt:  [−42.985% −41.341% −39.659%]
2026-08-02T22:20:47.8375955Z                         Performance has regressed.
2026-08-02T22:20:47.8376511Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:20:47.8377001Z   2 (10.00%) low mild
2026-08-02T22:20:47.8381952Z Benchmarking full_cycle/1x1/100key_100msg/u64
2026-08-02T22:20:47.8382675Z Benchmarking full_cycle/1x1/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:20:48.6462645Z Benchmarking full_cycle/1x1/100key_100msg/u64: Collecting 20 samples in estimated 3.0414 s (960 iterations)
2026-08-02T22:20:51.6690091Z Benchmarking full_cycle/1x1/100key_100msg/u64: Analyzing
2026-08-02T22:20:51.7264175Z full_cycle/1x1/100key_100msg/u64
2026-08-02T22:20:51.7264723Z                         time:   [3.1435 ms 3.1471 ms 3.1521 ms]
2026-08-02T22:20:51.7265788Z                         thrpt:  [28.616 Melem/s 28.661 Melem/s 28.694 Melem/s]
2026-08-02T22:20:51.7266175Z                  change:
2026-08-02T22:20:51.7266466Z                         time:   [+79.985% +80.350% +80.774%] (p = 0.00 < 0.05)
2026-08-02T22:20:51.7267079Z                         thrpt:  [−44.682% −44.552% −44.440%]
2026-08-02T22:20:51.7267445Z                         Performance has regressed.
2026-08-02T22:20:51.7268024Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:20:51.7268506Z   1 (5.00%) high mild
2026-08-02T22:20:51.7268719Z   1 (5.00%) high severe
2026-08-02T22:20:51.7271602Z Benchmarking full_cycle/1x1/100key_100msg/String
2026-08-02T22:20:51.7272564Z Benchmarking full_cycle/1x1/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:20:52.7319427Z Benchmarking full_cycle/1x1/100key_100msg/String: Collecting 20 samples in estimated 3.0059 s (380 iterations)
2026-08-02T22:20:55.8301928Z Benchmarking full_cycle/1x1/100key_100msg/String: Analyzing
2026-08-02T22:20:55.8894996Z full_cycle/1x1/100key_100msg/String
2026-08-02T22:20:55.8895609Z                         time:   [8.1117 ms 8.1513 ms 8.1873 ms]
2026-08-02T22:20:55.8896257Z                         thrpt:  [11.017 Melem/s 11.066 Melem/s 11.120 Melem/s]
2026-08-02T22:20:55.8896834Z                  change:
2026-08-02T22:20:55.8897313Z                         time:   [+72.839% +75.083% +77.525%] (p = 0.00 < 0.05)
2026-08-02T22:20:55.8898269Z                         thrpt:  [−43.670% −42.884% −42.143%]
2026-08-02T22:20:55.8898854Z                         Performance has regressed.
2026-08-02T22:20:55.8899399Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:20:55.8899912Z   1 (5.00%) low mild
2026-08-02T22:20:55.8902675Z Benchmarking full_cycle/1x1/100key_100msg/DropValue
2026-08-02T22:20:55.8903493Z Benchmarking full_cycle/1x1/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:20:56.7275527Z Benchmarking full_cycle/1x1/100key_100msg/DropValue: Collecting 20 samples in estimated 3.0202 s (920 iterations)
2026-08-02T22:20:59.7400908Z Benchmarking full_cycle/1x1/100key_100msg/DropValue: Analyzing
2026-08-02T22:20:59.7974105Z full_cycle/1x1/100key_100msg/DropValue
2026-08-02T22:20:59.7974571Z                         time:   [3.2674 ms 3.2729 ms 3.2781 ms]
2026-08-02T22:20:59.7974981Z                         thrpt:  [27.516 Melem/s 27.560 Melem/s 27.606 Melem/s]
2026-08-02T22:20:59.7975641Z                  change:
2026-08-02T22:20:59.7976569Z                         time:   [+77.590% +81.552% +83.930%] (p = 0.00 < 0.05)
2026-08-02T22:20:59.7978641Z                         thrpt:  [−45.631% −44.919% −43.691%]
2026-08-02T22:20:59.7979903Z                         Performance has regressed.
2026-08-02T22:20:59.7985003Z Benchmarking full_cycle/1x1/1000key_10msg/u64
2026-08-02T22:20:59.7985604Z Benchmarking full_cycle/1x1/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:21:00.7556756Z Benchmarking full_cycle/1x1/1000key_10msg/u64: Collecting 20 samples in estimated 3.0023 s (800 iterations)
2026-08-02T22:21:03.7710423Z Benchmarking full_cycle/1x1/1000key_10msg/u64: Analyzing
2026-08-02T22:21:03.8298193Z full_cycle/1x1/1000key_10msg/u64
2026-08-02T22:21:03.8298770Z                         time:   [3.7582 ms 3.7675 ms 3.7762 ms]
2026-08-02T22:21:03.8299420Z                         thrpt:  [24.363 Melem/s 24.419 Melem/s 24.480 Melem/s]
2026-08-02T22:21:03.8300019Z                  change:
2026-08-02T22:21:03.8300506Z                         time:   [+80.481% +81.003% +81.470%] (p = 0.00 < 0.05)
2026-08-02T22:21:03.8301623Z                         thrpt:  [−44.895% −44.752% −44.592%]
2026-08-02T22:21:03.8302174Z                         Performance has regressed.
2026-08-02T22:21:03.8306650Z Benchmarking full_cycle/1x1/1000key_10msg/String
2026-08-02T22:21:03.8307396Z Benchmarking full_cycle/1x1/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:21:04.3814747Z Benchmarking full_cycle/1x1/1000key_10msg/String: Collecting 20 samples in estimated 3.1466 s (360 iterations)
2026-08-02T22:21:07.6209599Z Benchmarking full_cycle/1x1/1000key_10msg/String: Analyzing
2026-08-02T22:21:07.6781783Z full_cycle/1x1/1000key_10msg/String
2026-08-02T22:21:07.6782415Z                         time:   [8.9665 ms 8.9963 ms 9.0219 ms]
2026-08-02T22:21:07.6783072Z                         thrpt:  [10.197 Melem/s 10.226 Melem/s 10.260 Melem/s]
2026-08-02T22:21:07.6783667Z                  change:
2026-08-02T22:21:07.6784165Z                         time:   [+84.107% +85.305% +86.438%] (p = 0.00 < 0.05)
2026-08-02T22:21:07.6785096Z                         thrpt:  [−46.363% −46.035% −45.684%]
2026-08-02T22:21:07.6785622Z                         Performance has regressed.
2026-08-02T22:21:07.6786168Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:21:07.6786668Z   1 (5.00%) low mild
2026-08-02T22:21:07.6790729Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue
2026-08-02T22:21:07.6791740Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:21:08.6819353Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0671 s (780 iterations)
2026-08-02T22:21:11.7466372Z Benchmarking full_cycle/1x1/1000key_10msg/DropValue: Analyzing
2026-08-02T22:21:11.8039516Z full_cycle/1x1/1000key_10msg/DropValue
2026-08-02T22:21:11.8040154Z                         time:   [3.9204 ms 3.9273 ms 3.9339 ms]
2026-08-02T22:21:11.8040811Z                         thrpt:  [23.386 Melem/s 23.426 Melem/s 23.467 Melem/s]
2026-08-02T22:21:11.8041636Z                  change:
2026-08-02T22:21:11.8042129Z                         time:   [+80.851% +81.294% +81.702%] (p = 0.00 < 0.05)
2026-08-02T22:21:11.8043067Z                         thrpt:  [−44.965% −44.841% −44.706%]
2026-08-02T22:21:11.8043621Z                         Performance has regressed.
2026-08-02T22:21:11.8048093Z Benchmarking full_cycle/1x1/10000key_1msg/u64
2026-08-02T22:21:11.8048801Z Benchmarking full_cycle/1x1/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:21:12.3168155Z Benchmarking full_cycle/1x1/10000key_1msg/u64: Collecting 20 samples in estimated 3.0874 s (380 iterations)
2026-08-02T22:21:15.3912622Z Benchmarking full_cycle/1x1/10000key_1msg/u64: Analyzing
2026-08-02T22:21:15.4530803Z full_cycle/1x1/10000key_1msg/u64
2026-08-02T22:21:15.4531595Z                         time:   [8.0767 ms 8.0881 ms 8.1000 ms]
2026-08-02T22:21:15.4532292Z                         thrpt:  [13.580 Melem/s 13.600 Melem/s 13.620 Melem/s]
2026-08-02T22:21:15.4532889Z                  change:
2026-08-02T22:21:15.4533367Z                         time:   [+56.980% +57.300% +57.609%] (p = 0.00 < 0.05)
2026-08-02T22:21:15.4534292Z                         thrpt:  [−36.552% −36.427% −36.298%]
2026-08-02T22:21:15.4534775Z                         Performance has regressed.
2026-08-02T22:21:15.4539088Z Benchmarking full_cycle/1x1/10000key_1msg/String
2026-08-02T22:21:15.4539841Z Benchmarking full_cycle/1x1/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:21:16.2410420Z Benchmarking full_cycle/1x1/10000key_1msg/String: Collecting 20 samples in estimated 3.2478 s (260 iterations)
2026-08-02T22:21:19.4995335Z Benchmarking full_cycle/1x1/10000key_1msg/String: Analyzing
2026-08-02T22:21:19.5556818Z full_cycle/1x1/10000key_1msg/String
2026-08-02T22:21:19.5557201Z                         time:   [12.482 ms 12.530 ms 12.615 ms]
2026-08-02T22:21:19.5557589Z                         thrpt:  [8.7198 Melem/s 8.7789 Melem/s 8.8128 Melem/s]
2026-08-02T22:21:19.5557919Z                  change:
2026-08-02T22:21:19.5559259Z                         time:   [+59.682% +60.363% +61.720%] (p = 0.00 < 0.05)
2026-08-02T22:21:19.5559866Z                         thrpt:  [−38.165% −37.641% −37.375%]
2026-08-02T22:21:19.5560189Z                         Performance has regressed.
2026-08-02T22:21:19.5560640Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:21:19.5560930Z   1 (5.00%) high severe
2026-08-02T22:21:19.5567437Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue
2026-08-02T22:21:19.5568039Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:21:20.0737160Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.1167 s (380 iterations)
2026-08-02T22:21:23.2439036Z Benchmarking full_cycle/1x1/10000key_1msg/DropValue: Analyzing
2026-08-02T22:21:23.2998475Z full_cycle/1x1/10000key_1msg/DropValue
2026-08-02T22:21:23.2999047Z                         time:   [8.2268 ms 8.3406 ms 8.5325 ms]
2026-08-02T22:21:23.2999691Z                         thrpt:  [12.892 Melem/s 13.188 Melem/s 13.371 Melem/s]
2026-08-02T22:21:23.3000495Z                  change:
2026-08-02T22:21:23.3001138Z                         time:   [+56.604% +58.820% +62.077%] (p = 0.00 < 0.05)
2026-08-02T22:21:23.3002497Z                         thrpt:  [−38.301% −37.036% −36.144%]
2026-08-02T22:21:23.3003162Z                         Performance has regressed.
2026-08-02T22:21:23.3003768Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:21:23.3004311Z   2 (10.00%) high severe
2026-08-02T22:21:23.3008036Z Benchmarking full_cycle/16x1/1key_10000msg/u64
2026-08-02T22:21:23.3008778Z Benchmarking full_cycle/16x1/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:21:24.0489504Z Benchmarking full_cycle/16x1/1key_10000msg/u64: Collecting 20 samples in estimated 3.0505 s (1040 iterations)
2026-08-02T22:21:27.0944384Z Benchmarking full_cycle/16x1/1key_10000msg/u64: Analyzing
2026-08-02T22:21:27.1520265Z full_cycle/16x1/1key_10000msg/u64
2026-08-02T22:21:27.1520638Z                         time:   [2.9208 ms 2.9267 ms 2.9320 ms]
2026-08-02T22:21:27.1521026Z                         thrpt:  [30.697 Melem/s 30.752 Melem/s 30.814 Melem/s]
2026-08-02T22:21:27.1521649Z                  change:
2026-08-02T22:21:27.1522125Z                         time:   [+81.471% +81.940% +82.391%] (p = 0.00 < 0.05)
2026-08-02T22:21:27.1523035Z                         thrpt:  [−45.173% −45.037% −44.895%]
2026-08-02T22:21:27.1523573Z                         Performance has regressed.
2026-08-02T22:21:27.1524174Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:21:27.1524735Z   1 (5.00%) low mild
2026-08-02T22:21:27.1527746Z Benchmarking full_cycle/16x1/1key_10000msg/String
2026-08-02T22:21:27.1528481Z Benchmarking full_cycle/16x1/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:21:27.6566547Z Benchmarking full_cycle/16x1/1key_10000msg/String: Collecting 20 samples in estimated 3.0383 s (380 iterations)
2026-08-02T22:21:30.7431244Z Benchmarking full_cycle/16x1/1key_10000msg/String: Analyzing
2026-08-02T22:21:30.8056076Z full_cycle/16x1/1key_10000msg/String
2026-08-02T22:21:30.8056690Z                         time:   [8.0733 ms 8.1200 ms 8.1627 ms]
2026-08-02T22:21:30.8057341Z                         thrpt:  [11.026 Melem/s 11.084 Melem/s 11.148 Melem/s]
2026-08-02T22:21:30.8057932Z                  change:
2026-08-02T22:21:30.8058438Z                         time:   [+79.795% +81.309% +82.625%] (p = 0.00 < 0.05)
2026-08-02T22:21:30.8059406Z                         thrpt:  [−45.243% −44.846% −44.381%]
2026-08-02T22:21:30.8060031Z                         Performance has regressed.
2026-08-02T22:21:30.8060596Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:21:30.8061145Z   2 (10.00%) low mild
2026-08-02T22:21:30.8066606Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue
2026-08-02T22:21:30.8067392Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:21:31.5812782Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.0367 s (1000 iterations)
2026-08-02T22:21:34.6292756Z Benchmarking full_cycle/16x1/1key_10000msg/DropValue: Analyzing
2026-08-02T22:21:34.6859123Z full_cycle/16x1/1key_10000msg/DropValue
2026-08-02T22:21:34.6859756Z                         time:   [3.0390 ms 3.0463 ms 3.0524 ms]
2026-08-02T22:21:34.6860413Z                         thrpt:  [29.485 Melem/s 29.545 Melem/s 29.615 Melem/s]
2026-08-02T22:21:34.6861005Z                  change:
2026-08-02T22:21:34.6861733Z                         time:   [+79.292% +80.881% +81.868%] (p = 0.00 < 0.05)
2026-08-02T22:21:34.6862972Z                         thrpt:  [−45.015% −44.715% −44.225%]
2026-08-02T22:21:34.6863497Z                         Performance has regressed.
2026-08-02T22:21:34.6864050Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:21:34.6864558Z   2 (10.00%) low mild
2026-08-02T22:21:34.6867102Z Benchmarking full_cycle/16x1/10key_1000msg/u64
2026-08-02T22:21:34.6867811Z Benchmarking full_cycle/16x1/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:21:35.4375259Z Benchmarking full_cycle/16x1/10key_1000msg/u64: Collecting 20 samples in estimated 3.0025 s (1020 iterations)
2026-08-02T22:21:38.4380912Z Benchmarking full_cycle/16x1/10key_1000msg/u64: Analyzing
2026-08-02T22:21:38.4948702Z full_cycle/16x1/10key_1000msg/u64
2026-08-02T22:21:38.4949177Z                         time:   [2.9355 ms 2.9401 ms 2.9449 ms]
2026-08-02T22:21:38.4949704Z                         thrpt:  [30.568 Melem/s 30.618 Melem/s 30.666 Melem/s]
2026-08-02T22:21:38.4950618Z                  change:
2026-08-02T22:21:38.4950988Z                         time:   [+82.306% +82.763% +83.254%] (p = 0.00 < 0.05)
2026-08-02T22:21:38.4952036Z                         thrpt:  [−45.431% −45.284% −45.147%]
2026-08-02T22:21:38.4952492Z                         Performance has regressed.
2026-08-02T22:21:38.4955515Z Benchmarking full_cycle/16x1/10key_1000msg/String
2026-08-02T22:21:38.4955987Z Benchmarking full_cycle/16x1/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:21:39.0072980Z Benchmarking full_cycle/16x1/10key_1000msg/String: Collecting 20 samples in estimated 3.0855 s (380 iterations)
2026-08-02T22:21:42.1262527Z Benchmarking full_cycle/16x1/10key_1000msg/String: Analyzing
2026-08-02T22:21:42.1831090Z full_cycle/16x1/10key_1000msg/String
2026-08-02T22:21:42.1831711Z                         time:   [8.1582 ms 8.2056 ms 8.2472 ms]
2026-08-02T22:21:42.1832103Z                         thrpt:  [10.915 Melem/s 10.971 Melem/s 11.034 Melem/s]
2026-08-02T22:21:42.1832442Z                  change:
2026-08-02T22:21:42.1832764Z                         time:   [+69.428% +75.773% +80.944%] (p = 0.00 < 0.05)
2026-08-02T22:21:42.1833354Z                         thrpt:  [−44.734% −43.108% −40.978%]
2026-08-02T22:21:42.1833666Z                         Performance has regressed.
2026-08-02T22:21:42.1833997Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:21:42.1834283Z   2 (10.00%) low mild
2026-08-02T22:21:42.1838356Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue
2026-08-02T22:21:42.1839142Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:21:42.9672926Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0100 s (980 iterations)
2026-08-02T22:21:45.9776012Z Benchmarking full_cycle/16x1/10key_1000msg/DropValue: Analyzing
2026-08-02T22:21:46.0361171Z full_cycle/16x1/10key_1000msg/DropValue
2026-08-02T22:21:46.0362088Z                         time:   [3.0584 ms 3.0701 ms 3.0866 ms]
2026-08-02T22:21:46.0362697Z                         thrpt:  [29.165 Melem/s 29.321 Melem/s 29.434 Melem/s]
2026-08-02T22:21:46.0363167Z                  change:
2026-08-02T22:21:46.0363658Z                         time:   [+82.514% +83.153% +84.188%] (p = 0.00 < 0.05)
2026-08-02T22:21:46.0364569Z                         thrpt:  [−45.708% −45.401% −45.210%]
2026-08-02T22:21:46.0365097Z                         Performance has regressed.
2026-08-02T22:21:46.0365435Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:21:46.0365750Z   1 (5.00%) low severe
2026-08-02T22:21:46.0365959Z   3 (15.00%) low mild
2026-08-02T22:21:46.0366163Z   1 (5.00%) high severe
2026-08-02T22:21:46.0368987Z Benchmarking full_cycle/16x1/100key_100msg/u64
2026-08-02T22:21:46.0369576Z Benchmarking full_cycle/16x1/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:21:46.8530873Z Benchmarking full_cycle/16x1/100key_100msg/u64: Collecting 20 samples in estimated 3.0079 s (940 iterations)
2026-08-02T22:21:49.8297171Z Benchmarking full_cycle/16x1/100key_100msg/u64: Analyzing
2026-08-02T22:21:49.8924613Z full_cycle/16x1/100key_100msg/u64
2026-08-02T22:21:49.8925225Z                         time:   [3.1618 ms 3.1650 ms 3.1680 ms]
2026-08-02T22:21:49.8925860Z                         thrpt:  [28.472 Melem/s 28.499 Melem/s 28.528 Melem/s]
2026-08-02T22:21:49.8926435Z                  change:
2026-08-02T22:21:49.8926905Z                         time:   [+81.477% +81.748% +82.033%] (p = 0.00 < 0.05)
2026-08-02T22:21:49.8927858Z                         thrpt:  [−45.065% −44.979% −44.897%]
2026-08-02T22:21:49.8928411Z                         Performance has regressed.
2026-08-02T22:21:49.8931179Z Benchmarking full_cycle/16x1/100key_100msg/String
2026-08-02T22:21:49.8932084Z Benchmarking full_cycle/16x1/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:21:50.4162609Z Benchmarking full_cycle/16x1/100key_100msg/String: Collecting 20 samples in estimated 3.1545 s (380 iterations)
2026-08-02T22:21:53.6632071Z Benchmarking full_cycle/16x1/100key_100msg/String: Analyzing
2026-08-02T22:21:53.7207916Z full_cycle/16x1/100key_100msg/String
2026-08-02T22:21:53.7208535Z                         time:   [8.5176 ms 8.5422 ms 8.5661 ms]
2026-08-02T22:21:53.7209214Z                         thrpt:  [10.530 Melem/s 10.559 Melem/s 10.590 Melem/s]
2026-08-02T22:21:53.7209829Z                  change:
2026-08-02T22:21:53.7210335Z                         time:   [+75.058% +77.859% +80.183%] (p = 0.00 < 0.05)
2026-08-02T22:21:53.7211587Z                         thrpt:  [−44.501% −43.776% −42.876%]
2026-08-02T22:21:53.7212157Z                         Performance has regressed.
2026-08-02T22:21:53.7216369Z Benchmarking full_cycle/16x1/100key_100msg/DropValue
2026-08-02T22:21:53.7217152Z Benchmarking full_cycle/16x1/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:21:54.5629461Z Benchmarking full_cycle/16x1/100key_100msg/DropValue: Collecting 20 samples in estimated 3.0346 s (920 iterations)
2026-08-02T22:21:57.5780508Z Benchmarking full_cycle/16x1/100key_100msg/DropValue: Analyzing
2026-08-02T22:21:57.6367188Z full_cycle/16x1/100key_100msg/DropValue
2026-08-02T22:21:57.6367762Z                         time:   [3.2706 ms 3.2756 ms 3.2799 ms]
2026-08-02T22:21:57.6368153Z                         thrpt:  [27.500 Melem/s 27.537 Melem/s 27.579 Melem/s]
2026-08-02T22:21:57.6368695Z                  change:
2026-08-02T22:21:57.6369165Z                         time:   [+79.711% +80.038% +80.325%] (p = 0.00 < 0.05)
2026-08-02T22:21:57.6370230Z                         thrpt:  [−44.544% −44.456% −44.355%]
2026-08-02T22:21:57.6370572Z                         Performance has regressed.
2026-08-02T22:21:57.6370904Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:21:57.6371474Z   1 (5.00%) low severe
2026-08-02T22:21:57.6371868Z   3 (15.00%) low mild
2026-08-02T22:21:57.6374290Z Benchmarking full_cycle/16x1/1000key_10msg/u64
2026-08-02T22:21:57.6374714Z Benchmarking full_cycle/16x1/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:21:58.6036792Z Benchmarking full_cycle/16x1/1000key_10msg/u64: Collecting 20 samples in estimated 3.0309 s (800 iterations)
2026-08-02T22:22:01.6350157Z Benchmarking full_cycle/16x1/1000key_10msg/u64: Analyzing
2026-08-02T22:22:01.6937721Z full_cycle/16x1/1000key_10msg/u64
2026-08-02T22:22:01.6938269Z                         time:   [3.7825 ms 3.7874 ms 3.7920 ms]
2026-08-02T22:22:01.6938665Z                         thrpt:  [24.262 Melem/s 24.291 Melem/s 24.322 Melem/s]
2026-08-02T22:22:01.6939143Z                  change:
2026-08-02T22:22:01.6939525Z                         time:   [+78.164% +78.517% +78.888%] (p = 0.00 < 0.05)
2026-08-02T22:22:01.6940173Z                         thrpt:  [−44.099% −43.983% −43.872%]
2026-08-02T22:22:01.6940521Z                         Performance has regressed.
2026-08-02T22:22:01.6943251Z Benchmarking full_cycle/16x1/1000key_10msg/String
2026-08-02T22:22:01.6943865Z Benchmarking full_cycle/16x1/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:22:02.2993387Z Benchmarking full_cycle/16x1/1000key_10msg/String: Collecting 20 samples in estimated 3.0718 s (320 iterations)
2026-08-02T22:22:05.2385152Z Benchmarking full_cycle/16x1/1000key_10msg/String: Analyzing
2026-08-02T22:22:05.2959311Z full_cycle/16x1/1000key_10msg/String
2026-08-02T22:22:05.2959934Z                         time:   [9.1375 ms 9.1827 ms 9.2243 ms]
2026-08-02T22:22:05.2960587Z                         thrpt:  [9.9737 Melem/s 10.019 Melem/s 10.068 Melem/s]
2026-08-02T22:22:05.2961178Z                  change:
2026-08-02T22:22:05.2961919Z                         time:   [+79.823% +81.116% +82.156%] (p = 0.00 < 0.05)
2026-08-02T22:22:05.2962855Z                         thrpt:  [−45.102% −44.787% −44.390%]
2026-08-02T22:22:05.2963368Z                         Performance has regressed.
2026-08-02T22:22:05.2963875Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:05.2964324Z   1 (5.00%) low severe
2026-08-02T22:22:05.2964640Z   2 (10.00%) low mild
2026-08-02T22:22:05.2965026Z   1 (5.00%) high severe
2026-08-02T22:22:05.2968744Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue
2026-08-02T22:22:05.2969549Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:22:05.8029726Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0273 s (760 iterations)
2026-08-02T22:22:08.8148017Z Benchmarking full_cycle/16x1/1000key_10msg/DropValue: Analyzing
2026-08-02T22:22:08.8712586Z full_cycle/16x1/1000key_10msg/DropValue
2026-08-02T22:22:08.8712969Z                         time:   [3.9550 ms 3.9612 ms 3.9669 ms]
2026-08-02T22:22:08.8713352Z                         thrpt:  [23.192 Melem/s 23.225 Melem/s 23.262 Melem/s]
2026-08-02T22:22:08.8713704Z                  change:
2026-08-02T22:22:08.8714176Z                         time:   [+74.184% +75.726% +77.186%] (p = 0.00 < 0.05)
2026-08-02T22:22:08.8714835Z                         thrpt:  [−43.562% −43.093% −42.589%]
2026-08-02T22:22:08.8715291Z                         Performance has regressed.
2026-08-02T22:22:08.8715653Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:22:08.8716115Z   1 (5.00%) low mild
2026-08-02T22:22:08.8721159Z Benchmarking full_cycle/16x1/10000key_1msg/u64
2026-08-02T22:22:08.8722067Z Benchmarking full_cycle/16x1/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:22:09.3841124Z Benchmarking full_cycle/16x1/10000key_1msg/u64: Collecting 20 samples in estimated 3.0871 s (380 iterations)
2026-08-02T22:22:12.5520671Z Benchmarking full_cycle/16x1/10000key_1msg/u64: Analyzing
2026-08-02T22:22:12.6113725Z full_cycle/16x1/10000key_1msg/u64
2026-08-02T22:22:12.6114299Z                         time:   [8.1414 ms 8.3342 ms 8.5737 ms]
2026-08-02T22:22:12.6115189Z                         thrpt:  [12.830 Melem/s 13.199 Melem/s 13.511 Melem/s]
2026-08-02T22:22:12.6115772Z                  change:
2026-08-02T22:22:12.6116249Z                         time:   [+56.308% +60.049% +64.430%] (p = 0.00 < 0.05)
2026-08-02T22:22:12.6117195Z                         thrpt:  [−39.184% −37.519% −36.024%]
2026-08-02T22:22:12.6117791Z                         Performance has regressed.
2026-08-02T22:22:12.6118403Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:12.6118783Z   4 (20.00%) high severe
2026-08-02T22:22:12.6124827Z Benchmarking full_cycle/16x1/10000key_1msg/String
2026-08-02T22:22:12.6125631Z Benchmarking full_cycle/16x1/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:22:13.4326653Z Benchmarking full_cycle/16x1/10000key_1msg/String: Collecting 20 samples in estimated 3.1239 s (240 iterations)
2026-08-02T22:22:16.5554578Z Benchmarking full_cycle/16x1/10000key_1msg/String: Analyzing
2026-08-02T22:22:16.6134826Z full_cycle/16x1/10000key_1msg/String
2026-08-02T22:22:16.6135378Z                         time:   [12.995 ms 13.009 ms 13.025 ms]
2026-08-02T22:22:16.6135789Z                         thrpt:  [8.4455 Melem/s 8.4557 Melem/s 8.4645 Melem/s]
2026-08-02T22:22:16.6136469Z                  change:
2026-08-02T22:22:16.6136768Z                         time:   [+63.201% +63.458% +63.719%] (p = 0.00 < 0.05)
2026-08-02T22:22:16.6138163Z                         thrpt:  [−38.920% −38.822% −38.726%]
2026-08-02T22:22:16.6138544Z                         Performance has regressed.
2026-08-02T22:22:16.6138894Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:22:16.6139455Z   1 (5.00%) high mild
2026-08-02T22:22:16.6140748Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue
2026-08-02T22:22:16.6141724Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:22:17.1382720Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.1606 s (380 iterations)
2026-08-02T22:22:20.2835470Z Benchmarking full_cycle/16x1/10000key_1msg/DropValue: Analyzing
2026-08-02T22:22:20.3411034Z full_cycle/16x1/10000key_1msg/DropValue
2026-08-02T22:22:20.3412016Z                         time:   [8.2667 ms 8.2746 ms 8.2844 ms]
2026-08-02T22:22:20.3412636Z                         thrpt:  [13.278 Melem/s 13.294 Melem/s 13.306 Melem/s]
2026-08-02T22:22:20.3414044Z                  change:
2026-08-02T22:22:20.3414787Z                         time:   [+57.206% +57.413% +57.634%] (p = 0.00 < 0.05)
2026-08-02T22:22:20.3415689Z                         thrpt:  [−36.562% −36.473% −36.389%]
2026-08-02T22:22:20.3416019Z                         Performance has regressed.
2026-08-02T22:22:20.3416565Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:22:20.3416861Z   1 (5.00%) high mild
2026-08-02T22:22:20.3417073Z   1 (5.00%) high severe
2026-08-02T22:22:20.3419873Z Benchmarking full_cycle/1x16/1key_10000msg/u64
2026-08-02T22:22:20.3420419Z Benchmarking full_cycle/1x16/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:22:20.9052136Z Benchmarking full_cycle/1x16/1key_10000msg/u64: Collecting 20 samples in estimated 3.2691 s (180 iterations)
2026-08-02T22:22:24.1663827Z Benchmarking full_cycle/1x16/1key_10000msg/u64: Analyzing
2026-08-02T22:22:24.2227951Z full_cycle/1x16/1key_10000msg/u64
2026-08-02T22:22:24.2228528Z                         time:   [18.055 ms 18.115 ms 18.164 ms]
2026-08-02T22:22:24.2229178Z                         thrpt:  [37.988 Melem/s 38.091 Melem/s 38.217 Melem/s]
2026-08-02T22:22:24.2229766Z                  change:
2026-08-02T22:22:24.2230234Z                         time:   [+90.489% +91.646% +92.983%] (p = 0.00 < 0.05)
2026-08-02T22:22:24.2231153Z                         thrpt:  [−48.182% −47.820% −47.504%]
2026-08-02T22:22:24.2231913Z                         Performance has regressed.
2026-08-02T22:22:24.2232510Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:24.2232861Z   4 (20.00%) low severe
2026-08-02T22:22:24.2237375Z Benchmarking full_cycle/1x16/1key_10000msg/String
2026-08-02T22:22:24.2238188Z Benchmarking full_cycle/1x16/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:22:24.7335684Z Benchmarking full_cycle/1x16/1key_10000msg/String: Collecting 20 samples in estimated 3.3979 s (100 iterations)
2026-08-02T22:22:28.1449713Z Benchmarking full_cycle/1x16/1key_10000msg/String: Analyzing
2026-08-02T22:22:28.2017585Z full_cycle/1x16/1key_10000msg/String
2026-08-02T22:22:28.2018161Z                         time:   [33.970 ms 34.109 ms 34.300 ms]
2026-08-02T22:22:28.2018797Z                         thrpt:  [20.118 Melem/s 20.230 Melem/s 20.313 Melem/s]
2026-08-02T22:22:28.2019933Z                  change:
2026-08-02T22:22:28.2020692Z                         time:   [+93.616% +94.538% +95.737%] (p = 0.00 < 0.05)
2026-08-02T22:22:28.2022039Z                         thrpt:  [−48.911% −48.596% −48.351%]
2026-08-02T22:22:28.2022408Z                         Performance has regressed.
2026-08-02T22:22:28.2022747Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:22:28.2023246Z   4 (20.00%) low mild
2026-08-02T22:22:28.2023581Z   1 (5.00%) high severe
2026-08-02T22:22:28.2025075Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue
2026-08-02T22:22:28.2025722Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:22:28.8236011Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.2045 s (160 iterations)
2026-08-02T22:22:32.0300330Z Benchmarking full_cycle/1x16/1key_10000msg/DropValue: Analyzing
2026-08-02T22:22:32.0876055Z full_cycle/1x16/1key_10000msg/DropValue
2026-08-02T22:22:32.0876664Z                         time:   [19.953 ms 20.037 ms 20.105 ms]
2026-08-02T22:22:32.0877311Z                         thrpt:  [34.322 Melem/s 34.438 Melem/s 34.582 Melem/s]
2026-08-02T22:22:32.0877887Z                  change:
2026-08-02T22:22:32.0878410Z                         time:   [+100.06% +100.83% +101.49%] (p = 0.00 < 0.05)
2026-08-02T22:22:32.0879339Z                         thrpt:  [−50.369% −50.207% −50.014%]
2026-08-02T22:22:32.0879916Z                         Performance has regressed.
2026-08-02T22:22:32.0880470Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:32.0880778Z   4 (20.00%) low severe
2026-08-02T22:22:32.0884402Z Benchmarking full_cycle/1x16/10key_1000msg/u64
2026-08-02T22:22:32.0885469Z Benchmarking full_cycle/1x16/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:22:32.6498284Z Benchmarking full_cycle/1x16/10key_1000msg/u64: Collecting 20 samples in estimated 3.2588 s (180 iterations)
2026-08-02T22:22:35.8985748Z Benchmarking full_cycle/1x16/10key_1000msg/u64: Analyzing
2026-08-02T22:22:35.9546734Z full_cycle/1x16/10key_1000msg/u64
2026-08-02T22:22:35.9547294Z                         time:   [17.985 ms 18.046 ms 18.095 ms]
2026-08-02T22:22:35.9547852Z                         thrpt:  [38.149 Melem/s 38.254 Melem/s 38.383 Melem/s]
2026-08-02T22:22:35.9548334Z                  change:
2026-08-02T22:22:35.9548719Z                         time:   [+90.664% +91.848% +93.172%] (p = 0.00 < 0.05)
2026-08-02T22:22:35.9549522Z                         thrpt:  [−48.233% −47.875% −47.552%]
2026-08-02T22:22:35.9550068Z                         Performance has regressed.
2026-08-02T22:22:35.9550570Z Found 7 outliers among 20 measurements (35.00%)
2026-08-02T22:22:35.9550938Z   4 (20.00%) low severe
2026-08-02T22:22:35.9551157Z   2 (10.00%) high mild
2026-08-02T22:22:35.9551725Z   1 (5.00%) high severe
2026-08-02T22:22:35.9553638Z Benchmarking full_cycle/1x16/10key_1000msg/String
2026-08-02T22:22:35.9554457Z Benchmarking full_cycle/1x16/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:22:36.4623927Z Benchmarking full_cycle/1x16/10key_1000msg/String: Collecting 20 samples in estimated 3.3791 s (100 iterations)
2026-08-02T22:22:39.8666149Z Benchmarking full_cycle/1x16/10key_1000msg/String: Analyzing
2026-08-02T22:22:39.9268401Z full_cycle/1x16/10key_1000msg/String
2026-08-02T22:22:39.9268972Z                         time:   [33.981 ms 34.038 ms 34.090 ms]
2026-08-02T22:22:39.9269371Z                         thrpt:  [20.250 Melem/s 20.281 Melem/s 20.315 Melem/s]
2026-08-02T22:22:39.9269850Z                  change:
2026-08-02T22:22:39.9270360Z                         time:   [+94.049% +94.903% +95.567%] (p = 0.00 < 0.05)
2026-08-02T22:22:39.9271575Z                         thrpt:  [−48.867% −48.692% −48.467%]
2026-08-02T22:22:39.9272171Z                         Performance has regressed.
2026-08-02T22:22:39.9277645Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue
2026-08-02T22:22:39.9278467Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:22:40.5500520Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.2110 s (160 iterations)
2026-08-02T22:22:43.7427424Z Benchmarking full_cycle/1x16/10key_1000msg/DropValue: Analyzing
2026-08-02T22:22:43.7999721Z full_cycle/1x16/10key_1000msg/DropValue
2026-08-02T22:22:43.8000299Z                         time:   [19.868 ms 19.952 ms 20.020 ms]
2026-08-02T22:22:43.8000737Z                         thrpt:  [34.481 Melem/s 34.599 Melem/s 34.746 Melem/s]
2026-08-02T22:22:43.8001754Z                  change:
2026-08-02T22:22:43.8002342Z                         time:   [+93.407% +97.697% +101.03%] (p = 0.00 < 0.05)
2026-08-02T22:22:43.8003365Z                         thrpt:  [−50.255% −49.418% −48.296%]
2026-08-02T22:22:43.8004072Z                         Performance has regressed.
2026-08-02T22:22:43.8004432Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:43.8004746Z   4 (20.00%) low severe
2026-08-02T22:22:43.8006290Z Benchmarking full_cycle/1x16/100key_100msg/u64
2026-08-02T22:22:43.8006874Z Benchmarking full_cycle/1x16/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:22:44.3674836Z Benchmarking full_cycle/1x16/100key_100msg/u64: Collecting 20 samples in estimated 3.2905 s (180 iterations)
2026-08-02T22:22:47.6484078Z Benchmarking full_cycle/1x16/100key_100msg/u64: Analyzing
2026-08-02T22:22:47.7055824Z full_cycle/1x16/100key_100msg/u64
2026-08-02T22:22:47.7056406Z                         time:   [18.167 ms 18.225 ms 18.273 ms]
2026-08-02T22:22:47.7057063Z                         thrpt:  [37.936 Melem/s 38.036 Melem/s 38.157 Melem/s]
2026-08-02T22:22:47.7057681Z                  change:
2026-08-02T22:22:47.7058183Z                         time:   [+91.962% +93.691% +94.998%] (p = 0.00 < 0.05)
2026-08-02T22:22:47.7059414Z                         thrpt:  [−48.717% −48.371% −47.906%]
2026-08-02T22:22:47.7059941Z                         Performance has regressed.
2026-08-02T22:22:47.7060455Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:47.7060978Z   4 (20.00%) low severe
2026-08-02T22:22:47.7064764Z Benchmarking full_cycle/1x16/100key_100msg/String
2026-08-02T22:22:47.7065530Z Benchmarking full_cycle/1x16/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:22:48.2208208Z Benchmarking full_cycle/1x16/100key_100msg/String: Collecting 20 samples in estimated 3.4281 s (100 iterations)
2026-08-02T22:22:51.6524785Z Benchmarking full_cycle/1x16/100key_100msg/String: Analyzing
2026-08-02T22:22:51.7152542Z full_cycle/1x16/100key_100msg/String
2026-08-02T22:22:51.7153120Z                         time:   [34.241 ms 34.311 ms 34.395 ms]
2026-08-02T22:22:51.7153757Z                         thrpt:  [20.154 Melem/s 20.203 Melem/s 20.245 Melem/s]
2026-08-02T22:22:51.7154367Z                  change:
2026-08-02T22:22:51.7154861Z                         time:   [+94.294% +94.843% +95.479%] (p = 0.00 < 0.05)
2026-08-02T22:22:51.7155774Z                         thrpt:  [−48.844% −48.677% −48.532%]
2026-08-02T22:22:51.7156323Z                         Performance has regressed.
2026-08-02T22:22:51.7156784Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:22:51.7157103Z   2 (10.00%) high mild
2026-08-02T22:22:51.7160819Z Benchmarking full_cycle/1x16/100key_100msg/DropValue
2026-08-02T22:22:51.7161855Z Benchmarking full_cycle/1x16/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:22:52.3330854Z Benchmarking full_cycle/1x16/100key_100msg/DropValue: Collecting 20 samples in estimated 3.1836 s (160 iterations)
2026-08-02T22:22:55.5110826Z Benchmarking full_cycle/1x16/100key_100msg/DropValue: Analyzing
2026-08-02T22:22:55.5672703Z full_cycle/1x16/100key_100msg/DropValue
2026-08-02T22:22:55.5673349Z                         time:   [19.779 ms 19.860 ms 19.927 ms]
2026-08-02T22:22:55.5674700Z                         thrpt:  [34.786 Melem/s 34.905 Melem/s 35.047 Melem/s]
2026-08-02T22:22:55.5675354Z                  change:
2026-08-02T22:22:55.5675853Z                         time:   [+98.034% +98.901% +99.596%] (p = 0.00 < 0.05)
2026-08-02T22:22:55.5677128Z                         thrpt:  [−49.899% −49.724% −49.504%]
2026-08-02T22:22:55.5677920Z                         Performance has regressed.
2026-08-02T22:22:55.5678438Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:22:55.5678964Z   4 (20.00%) low severe
2026-08-02T22:22:55.5681526Z Benchmarking full_cycle/1x16/1000key_10msg/u64
2026-08-02T22:22:55.5682246Z Benchmarking full_cycle/1x16/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:22:56.1882336Z Benchmarking full_cycle/1x16/1000key_10msg/u64: Collecting 20 samples in estimated 3.1994 s (160 iterations)
2026-08-02T22:22:59.3843014Z Benchmarking full_cycle/1x16/1000key_10msg/u64: Analyzing
2026-08-02T22:22:59.4402941Z full_cycle/1x16/1000key_10msg/u64
2026-08-02T22:22:59.4403814Z                         time:   [19.910 ms 19.973 ms 20.026 ms]
2026-08-02T22:22:59.4404750Z                         thrpt:  [36.053 Melem/s 36.150 Melem/s 36.263 Melem/s]
2026-08-02T22:22:59.4405638Z                  change:
2026-08-02T22:22:59.4406151Z                         time:   [+87.499% +88.281% +89.076%] (p = 0.00 < 0.05)
2026-08-02T22:22:59.4407387Z                         thrpt:  [−47.111% −46.888% −46.666%]
2026-08-02T22:22:59.4407962Z                         Performance has regressed.
2026-08-02T22:22:59.4408539Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:22:59.4409030Z   4 (20.00%) low severe
2026-08-02T22:22:59.4409388Z   1 (5.00%) high severe
2026-08-02T22:22:59.4409820Z Benchmarking full_cycle/1x16/1000key_10msg/String
2026-08-02T22:22:59.4410567Z Benchmarking full_cycle/1x16/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:22:59.9828007Z Benchmarking full_cycle/1x16/1000key_10msg/String: Collecting 20 samples in estimated 3.6108 s (100 iterations)
2026-08-02T22:23:03.5740611Z Benchmarking full_cycle/1x16/1000key_10msg/String: Analyzing
2026-08-02T22:23:03.6323095Z full_cycle/1x16/1000key_10msg/String
2026-08-02T22:23:03.6323699Z                         time:   [35.870 ms 35.906 ms 35.942 ms]
2026-08-02T22:23:03.6324095Z                         thrpt:  [20.088 Melem/s 20.108 Melem/s 20.128 Melem/s]
2026-08-02T22:23:03.6324435Z                  change:
2026-08-02T22:23:03.6324947Z                         time:   [+89.559% +89.987% +90.451%] (p = 0.00 < 0.05)
2026-08-02T22:23:03.6325653Z                         thrpt:  [−47.493% −47.365% −47.246%]
2026-08-02T22:23:03.6325996Z                         Performance has regressed.
2026-08-02T22:23:03.6326374Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:23:03.6326694Z   1 (5.00%) low mild
2026-08-02T22:23:03.6326970Z   2 (10.00%) high mild
2026-08-02T22:23:03.6330289Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue
2026-08-02T22:23:03.6331113Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:23:04.3022440Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0212 s (140 iterations)
2026-08-02T22:23:07.3165791Z Benchmarking full_cycle/1x16/1000key_10msg/DropValue: Analyzing
2026-08-02T22:23:07.3789413Z full_cycle/1x16/1000key_10msg/DropValue
2026-08-02T22:23:07.3790039Z                         time:   [21.433 ms 21.528 ms 21.606 ms]
2026-08-02T22:23:07.3790927Z                         thrpt:  [33.416 Melem/s 33.538 Melem/s 33.686 Melem/s]
2026-08-02T22:23:07.3791867Z                  change:
2026-08-02T22:23:07.3792391Z                         time:   [+90.144% +90.964% +91.681%] (p = 0.00 < 0.05)
2026-08-02T22:23:07.3793447Z                         thrpt:  [−47.830% −47.634% −47.408%]
2026-08-02T22:23:07.3794125Z                         Performance has regressed.
2026-08-02T22:23:07.3794752Z Found 6 outliers among 20 measurements (30.00%)
2026-08-02T22:23:07.3795614Z   4 (20.00%) low severe
2026-08-02T22:23:07.3796165Z   2 (10.00%) high mild
2026-08-02T22:23:07.3813538Z Benchmarking full_cycle/1x16/10000key_1msg/u64
2026-08-02T22:23:07.3814321Z Benchmarking full_cycle/1x16/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:23:07.8998390Z Benchmarking full_cycle/1x16/10000key_1msg/u64: Collecting 20 samples in estimated 3.4644 s (100 iterations)
2026-08-02T22:23:11.3674957Z Benchmarking full_cycle/1x16/10000key_1msg/u64: Analyzing
2026-08-02T22:23:11.4240522Z full_cycle/1x16/10000key_1msg/u64
2026-08-02T22:23:11.4241107Z                         time:   [34.553 ms 34.670 ms 34.774 ms]
2026-08-02T22:23:11.4242548Z                         thrpt:  [29.045 Melem/s 29.132 Melem/s 29.230 Melem/s]
2026-08-02T22:23:11.4243136Z                  change:
2026-08-02T22:23:11.4243606Z                         time:   [+49.345% +50.004% +50.635%] (p = 0.00 < 0.05)
2026-08-02T22:23:11.4244890Z                         thrpt:  [−33.614% −33.335% −33.041%]
2026-08-02T22:23:11.4245297Z                         Performance has regressed.
2026-08-02T22:23:11.4245870Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:23:11.4246186Z   2 (10.00%) low mild
2026-08-02T22:23:11.4246753Z Benchmarking full_cycle/1x16/10000key_1msg/String
2026-08-02T22:23:11.4247495Z Benchmarking full_cycle/1x16/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:23:12.1650128Z Benchmarking full_cycle/1x16/10000key_1msg/String: Collecting 20 samples in estimated 3.9475 s (80 iterations)
2026-08-02T22:23:16.1232203Z Benchmarking full_cycle/1x16/10000key_1msg/String: Analyzing
2026-08-02T22:23:16.1833938Z full_cycle/1x16/10000key_1msg/String
2026-08-02T22:23:16.1834773Z                         time:   [49.374 ms 49.470 ms 49.585 ms]
2026-08-02T22:23:16.1835417Z                         thrpt:  [20.369 Melem/s 20.416 Melem/s 20.456 Melem/s]
2026-08-02T22:23:16.1836143Z                  change:
2026-08-02T22:23:16.1836628Z                         time:   [+64.131% +64.531% +64.980%] (p = 0.00 < 0.05)
2026-08-02T22:23:16.1837593Z                         thrpt:  [−39.387% −39.221% −39.073%]
2026-08-02T22:23:16.1838198Z                         Performance has regressed.
2026-08-02T22:23:16.1838588Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:23:16.1838883Z   1 (5.00%) high mild
2026-08-02T22:23:16.1839090Z   1 (5.00%) high severe
2026-08-02T22:23:16.1839408Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue
2026-08-02T22:23:16.1839846Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:23:16.7039044Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.4644 s (100 iterations)
2026-08-02T22:23:20.1487345Z Benchmarking full_cycle/1x16/10000key_1msg/DropValue: Analyzing
2026-08-02T22:23:20.2062425Z full_cycle/1x16/10000key_1msg/DropValue
2026-08-02T22:23:20.2063275Z                         time:   [34.370 ms 34.443 ms 34.510 ms]
2026-08-02T22:23:20.2063867Z                         thrpt:  [29.267 Melem/s 29.323 Melem/s 29.386 Melem/s]
2026-08-02T22:23:20.2064561Z                  change:
2026-08-02T22:23:20.2065069Z                         time:   [+51.745% +52.061% +52.406%] (p = 0.00 < 0.05)
2026-08-02T22:23:20.2065970Z                         thrpt:  [−34.386% −34.237% −34.100%]
2026-08-02T22:23:20.2066515Z                         Performance has regressed.
2026-08-02T22:23:20.2067046Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:23:20.2067564Z   2 (10.00%) low mild
2026-08-02T22:23:20.2068673Z Benchmarking full_cycle/4x4/1key_10000msg/u64
2026-08-02T22:23:20.2069357Z Benchmarking full_cycle/4x4/1key_10000msg/u64: Warming up for 500.00 ms
2026-08-02T22:23:20.9672830Z Benchmarking full_cycle/4x4/1key_10000msg/u64: Collecting 20 samples in estimated 3.1130 s (520 iterations)
2026-08-02T22:23:24.0657474Z Benchmarking full_cycle/4x4/1key_10000msg/u64: Analyzing
2026-08-02T22:23:24.1245569Z full_cycle/4x4/1key_10000msg/u64
2026-08-02T22:23:24.1246118Z                         time:   [5.9397 ms 5.9568 ms 5.9713 ms]
2026-08-02T22:23:24.1246560Z                         thrpt:  [35.169 Melem/s 35.255 Melem/s 35.357 Melem/s]
2026-08-02T22:23:24.1246901Z                  change:
2026-08-02T22:23:24.1247191Z                         time:   [+86.586% +87.461% +88.361%] (p = 0.00 < 0.05)
2026-08-02T22:23:24.1248055Z                         thrpt:  [−46.910% −46.655% −46.406%]
2026-08-02T22:23:24.1248384Z                         Performance has regressed.
2026-08-02T22:23:24.1248819Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:23:24.1249115Z   4 (20.00%) low severe
2026-08-02T22:23:24.1252136Z Benchmarking full_cycle/4x4/1key_10000msg/String
2026-08-02T22:23:24.1252864Z Benchmarking full_cycle/4x4/1key_10000msg/String: Warming up for 500.00 ms
2026-08-02T22:23:24.9408089Z Benchmarking full_cycle/4x4/1key_10000msg/String: Collecting 20 samples in estimated 3.1062 s (240 iterations)
2026-08-02T22:23:28.0386845Z Benchmarking full_cycle/4x4/1key_10000msg/String: Analyzing
2026-08-02T22:23:28.1010659Z full_cycle/4x4/1key_10000msg/String
2026-08-02T22:23:28.1011685Z                         time:   [12.846 ms 12.906 ms 12.957 ms]
2026-08-02T22:23:28.1012159Z                         thrpt:  [16.208 Melem/s 16.273 Melem/s 16.348 Melem/s]
2026-08-02T22:23:28.1012515Z                  change:
2026-08-02T22:23:28.1012804Z                         time:   [+84.293% +85.525% +86.571%] (p = 0.00 < 0.05)
2026-08-02T22:23:28.1013398Z                         thrpt:  [−46.401% −46.099% −45.739%]
2026-08-02T22:23:28.1013723Z                         Performance has regressed.
2026-08-02T22:23:28.1014050Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:23:28.1014337Z   3 (15.00%) low severe
2026-08-02T22:23:28.1014581Z   1 (5.00%) low mild
2026-08-02T22:23:28.1018033Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue
2026-08-02T22:23:28.1018667Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:23:28.9185475Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue: Collecting 20 samples in estimated 3.0862 s (480 iterations)
2026-08-02T22:23:32.0020895Z Benchmarking full_cycle/4x4/1key_10000msg/DropValue: Analyzing
2026-08-02T22:23:32.0605012Z full_cycle/4x4/1key_10000msg/DropValue
2026-08-02T22:23:32.0605608Z                         time:   [6.4012 ms 6.4219 ms 6.4393 ms]
2026-08-02T22:23:32.0606239Z                         thrpt:  [32.613 Melem/s 32.702 Melem/s 32.808 Melem/s]
2026-08-02T22:23:32.0606587Z                  change:
2026-08-02T22:23:32.0606970Z                         time:   [+91.923% +92.550% +93.068%] (p = 0.00 < 0.05)
2026-08-02T22:23:32.0607677Z                         thrpt:  [−48.205% −48.065% −47.896%]
2026-08-02T22:23:32.0608001Z                         Performance has regressed.
2026-08-02T22:23:32.0608434Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:23:32.0608728Z   4 (20.00%) low severe
2026-08-02T22:23:32.0612522Z Benchmarking full_cycle/4x4/10key_1000msg/u64
2026-08-02T22:23:32.0613210Z Benchmarking full_cycle/4x4/10key_1000msg/u64: Warming up for 500.00 ms
2026-08-02T22:23:32.8203402Z Benchmarking full_cycle/4x4/10key_1000msg/u64: Collecting 20 samples in estimated 3.1069 s (520 iterations)
2026-08-02T22:23:35.9362648Z Benchmarking full_cycle/4x4/10key_1000msg/u64: Analyzing
2026-08-02T22:23:35.9932901Z full_cycle/4x4/10key_1000msg/u64
2026-08-02T22:23:35.9935261Z                         time:   [5.9735 ms 5.9905 ms 6.0051 ms]
2026-08-02T22:23:35.9936089Z                         thrpt:  [34.984 Melem/s 35.069 Melem/s 35.168 Melem/s]
2026-08-02T22:23:35.9936473Z                  change:
2026-08-02T22:23:35.9936832Z                         time:   [+86.048% +87.909% +89.353%] (p = 0.00 < 0.05)
2026-08-02T22:23:35.9937753Z                         thrpt:  [−47.189% −46.783% −46.251%]
2026-08-02T22:23:35.9938095Z                         Performance has regressed.
2026-08-02T22:23:35.9938462Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:23:35.9938763Z   1 (5.00%) low severe
2026-08-02T22:23:35.9939006Z   3 (15.00%) low mild
2026-08-02T22:23:35.9941793Z Benchmarking full_cycle/4x4/10key_1000msg/String
2026-08-02T22:23:35.9942511Z Benchmarking full_cycle/4x4/10key_1000msg/String: Warming up for 500.00 ms
2026-08-02T22:23:36.8097603Z Benchmarking full_cycle/4x4/10key_1000msg/String: Collecting 20 samples in estimated 3.1062 s (240 iterations)
2026-08-02T22:23:39.9604107Z Benchmarking full_cycle/4x4/10key_1000msg/String: Analyzing
2026-08-02T22:23:40.0175812Z full_cycle/4x4/10key_1000msg/String
2026-08-02T22:23:40.0176399Z                         time:   [13.091 ms 13.124 ms 13.153 ms]
2026-08-02T22:23:40.0177028Z                         thrpt:  [15.972 Melem/s 16.008 Melem/s 16.048 Melem/s]
2026-08-02T22:23:40.0177584Z                  change:
2026-08-02T22:23:40.0178043Z                         time:   [+79.443% +84.517% +88.040%] (p = 0.00 < 0.05)
2026-08-02T22:23:40.0179004Z                         thrpt:  [−46.820% −45.804% −44.272%]
2026-08-02T22:23:40.0179583Z                         Performance has regressed.
2026-08-02T22:23:40.0180223Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:23:40.0180575Z   2 (10.00%) low severe
2026-08-02T22:23:40.0180791Z   1 (5.00%) low mild
2026-08-02T22:23:40.0180991Z   1 (5.00%) high mild
2026-08-02T22:23:40.0182079Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue
2026-08-02T22:23:40.0182851Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:23:40.8381400Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue: Collecting 20 samples in estimated 3.0982 s (480 iterations)
2026-08-02T22:23:43.9374190Z Benchmarking full_cycle/4x4/10key_1000msg/DropValue: Analyzing
2026-08-02T22:23:43.9950216Z full_cycle/4x4/10key_1000msg/DropValue
2026-08-02T22:23:43.9950797Z                         time:   [6.4400 ms 6.4547 ms 6.4665 ms]
2026-08-02T22:23:43.9951635Z                         thrpt:  [32.487 Melem/s 32.547 Melem/s 32.621 Melem/s]
2026-08-02T22:23:43.9952202Z                  change:
2026-08-02T22:23:43.9952678Z                         time:   [+91.952% +92.765% +93.464%] (p = 0.00 < 0.05)
2026-08-02T22:23:43.9953972Z                         thrpt:  [−48.311% −48.123% −47.904%]
2026-08-02T22:23:43.9954457Z                         Performance has regressed.
2026-08-02T22:23:43.9954894Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:23:43.9955194Z   2 (10.00%) low severe
2026-08-02T22:23:43.9955406Z   1 (5.00%) low mild
2026-08-02T22:23:43.9958614Z Benchmarking full_cycle/4x4/100key_100msg/u64
2026-08-02T22:23:43.9959315Z Benchmarking full_cycle/4x4/100key_100msg/u64: Warming up for 500.00 ms
2026-08-02T22:23:44.7891036Z Benchmarking full_cycle/4x4/100key_100msg/u64: Collecting 20 samples in estimated 3.1222 s (500 iterations)
2026-08-02T22:23:47.9078984Z Benchmarking full_cycle/4x4/100key_100msg/u64: Analyzing
2026-08-02T22:23:47.9666291Z full_cycle/4x4/100key_100msg/u64
2026-08-02T22:23:47.9666842Z                         time:   [6.2202 ms 6.2358 ms 6.2496 ms]
2026-08-02T22:23:47.9667478Z                         thrpt:  [33.730 Melem/s 33.805 Melem/s 33.889 Melem/s]
2026-08-02T22:23:47.9668112Z                  change:
2026-08-02T22:23:47.9668586Z                         time:   [+90.012% +90.690% +91.397%] (p = 0.00 < 0.05)
2026-08-02T22:23:47.9669580Z                         thrpt:  [−47.753% −47.559% −47.372%]
2026-08-02T22:23:47.9670148Z                         Performance has regressed.
2026-08-02T22:23:47.9670598Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:23:47.9670954Z   3 (15.00%) low severe
2026-08-02T22:23:47.9671165Z   1 (5.00%) low mild
2026-08-02T22:23:47.9673784Z Benchmarking full_cycle/4x4/100key_100msg/String
2026-08-02T22:23:47.9674489Z Benchmarking full_cycle/4x4/100key_100msg/String: Warming up for 500.00 ms
2026-08-02T22:23:48.8019360Z Benchmarking full_cycle/4x4/100key_100msg/String: Collecting 20 samples in estimated 3.1786 s (240 iterations)
2026-08-02T22:23:51.9771001Z Benchmarking full_cycle/4x4/100key_100msg/String: Analyzing
2026-08-02T22:23:52.0359016Z full_cycle/4x4/100key_100msg/String
2026-08-02T22:23:52.0359648Z                         time:   [13.199 ms 13.226 ms 13.251 ms]
2026-08-02T22:23:52.0360572Z                         thrpt:  [15.908 Melem/s 15.938 Melem/s 15.971 Melem/s]
2026-08-02T22:23:52.0361435Z                  change:
2026-08-02T22:23:52.0362192Z                         time:   [+84.490% +85.071% +85.606%] (p = 0.00 < 0.05)
2026-08-02T22:23:52.0363260Z                         thrpt:  [−46.123% −45.967% −45.796%]
2026-08-02T22:23:52.0363606Z                         Performance has regressed.
2026-08-02T22:23:52.0366824Z Benchmarking full_cycle/4x4/100key_100msg/DropValue
2026-08-02T22:23:52.0367580Z Benchmarking full_cycle/4x4/100key_100msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:23:52.8838639Z Benchmarking full_cycle/4x4/100key_100msg/DropValue: Collecting 20 samples in estimated 3.0680 s (460 iterations)
2026-08-02T22:23:55.9470376Z Benchmarking full_cycle/4x4/100key_100msg/DropValue: Analyzing
2026-08-02T22:23:56.0045874Z full_cycle/4x4/100key_100msg/DropValue
2026-08-02T22:23:56.0046768Z                         time:   [6.6344 ms 6.6570 ms 6.6764 ms]
2026-08-02T22:23:56.0047540Z                         thrpt:  [31.574 Melem/s 31.666 Melem/s 31.774 Melem/s]
2026-08-02T22:23:56.0048129Z                  change:
2026-08-02T22:23:56.0048522Z                         time:   [+91.591% +92.449% +93.216%] (p = 0.00 < 0.05)
2026-08-02T22:23:56.0049401Z                         thrpt:  [−48.244% −48.038% −47.806%]
2026-08-02T22:23:56.0049768Z                         Performance has regressed.
2026-08-02T22:23:56.0050100Z Found 5 outliers among 20 measurements (25.00%)
2026-08-02T22:23:56.0050425Z   4 (20.00%) low severe
2026-08-02T22:23:56.0050641Z   1 (5.00%) high mild
2026-08-02T22:23:56.0051876Z Benchmarking full_cycle/4x4/1000key_10msg/u64
2026-08-02T22:23:56.0052552Z Benchmarking full_cycle/4x4/1000key_10msg/u64: Warming up for 500.00 ms
2026-08-02T22:23:56.9071048Z Benchmarking full_cycle/4x4/1000key_10msg/u64: Collecting 20 samples in estimated 3.1241 s (440 iterations)
2026-08-02T22:24:00.0635259Z Benchmarking full_cycle/4x4/1000key_10msg/u64: Analyzing
2026-08-02T22:24:00.1210433Z full_cycle/4x4/1000key_10msg/u64
2026-08-02T22:24:00.1210992Z                         time:   [7.0629 ms 7.1714 ms 7.3438 ms]
2026-08-02T22:24:00.1211874Z                         thrpt:  [29.685 Melem/s 30.399 Melem/s 30.865 Melem/s]
2026-08-02T22:24:00.1212435Z                  change:
2026-08-02T22:24:00.1212896Z                         time:   [+83.755% +86.669% +91.282%] (p = 0.00 < 0.05)
2026-08-02T22:24:00.1213817Z                         thrpt:  [−47.721% −46.429% −45.580%]
2026-08-02T22:24:00.1214391Z                         Performance has regressed.
2026-08-02T22:24:00.1214951Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:24:00.1215438Z   1 (5.00%) low mild
2026-08-02T22:24:00.1215813Z   2 (10.00%) high severe
2026-08-02T22:24:00.1218648Z Benchmarking full_cycle/4x4/1000key_10msg/String
2026-08-02T22:24:00.1219089Z Benchmarking full_cycle/4x4/1000key_10msg/String: Warming up for 500.00 ms
2026-08-02T22:24:01.0260922Z Benchmarking full_cycle/4x4/1000key_10msg/String: Collecting 20 samples in estimated 3.1570 s (220 iterations)
2026-08-02T22:24:04.1511201Z Benchmarking full_cycle/4x4/1000key_10msg/String: Analyzing
2026-08-02T22:24:04.2083780Z full_cycle/4x4/1000key_10msg/String
2026-08-02T22:24:04.2084333Z                         time:   [14.189 ms 14.202 ms 14.216 ms]
2026-08-02T22:24:04.2084887Z                         thrpt:  [15.335 Melem/s 15.350 Melem/s 15.364 Melem/s]
2026-08-02T22:24:04.2085419Z                  change:
2026-08-02T22:24:04.2085731Z                         time:   [+84.933% +85.179% +85.409%] (p = 0.00 < 0.05)
2026-08-02T22:24:04.2086353Z                         thrpt:  [−46.065% −45.998% −45.926%]
2026-08-02T22:24:04.2086673Z                         Performance has regressed.
2026-08-02T22:24:04.2089786Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue
2026-08-02T22:24:04.2090602Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:24:05.1627903Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue: Collecting 20 samples in estimated 3.0037 s (400 iterations)
2026-08-02T22:24:08.1671568Z Benchmarking full_cycle/4x4/1000key_10msg/DropValue: Analyzing
2026-08-02T22:24:08.2242381Z full_cycle/4x4/1000key_10msg/DropValue
2026-08-02T22:24:08.2242978Z                         time:   [7.4847 ms 7.5087 ms 7.5290 ms]
2026-08-02T22:24:08.2243620Z                         thrpt:  [28.955 Melem/s 29.033 Melem/s 29.126 Melem/s]
2026-08-02T22:24:08.2244196Z                  change:
2026-08-02T22:24:08.2244682Z                         time:   [+85.094% +85.755% +86.293%] (p = 0.00 < 0.05)
2026-08-02T22:24:08.2245658Z                         thrpt:  [−46.321% −46.166% −45.973%]
2026-08-02T22:24:08.2246127Z                         Performance has regressed.
2026-08-02T22:24:08.2246463Z Found 4 outliers among 20 measurements (20.00%)
2026-08-02T22:24:08.2246784Z   4 (20.00%) low severe
2026-08-02T22:24:08.2249841Z Benchmarking full_cycle/4x4/10000key_1msg/u64
2026-08-02T22:24:08.2250663Z Benchmarking full_cycle/4x4/10000key_1msg/u64: Warming up for 500.00 ms
2026-08-02T22:24:09.0687131Z Benchmarking full_cycle/4x4/10000key_1msg/u64: Collecting 20 samples in estimated 3.2136 s (240 iterations)
2026-08-02T22:24:12.2502503Z Benchmarking full_cycle/4x4/10000key_1msg/u64: Analyzing
2026-08-02T22:24:12.3073075Z full_cycle/4x4/10000key_1msg/u64
2026-08-02T22:24:12.3073450Z                         time:   [13.237 ms 13.254 ms 13.268 ms]
2026-08-02T22:24:12.3073842Z                         thrpt:  [21.858 Melem/s 21.881 Melem/s 21.908 Melem/s]
2026-08-02T22:24:12.3074381Z                  change:
2026-08-02T22:24:12.3074680Z                         time:   [+53.444% +53.786% +54.122%] (p = 0.00 < 0.05)
2026-08-02T22:24:12.3075457Z                         thrpt:  [−35.116% −34.975% −34.830%]
2026-08-02T22:24:12.3075922Z                         Performance has regressed.
2026-08-02T22:24:12.3076320Z Found 3 outliers among 20 measurements (15.00%)
2026-08-02T22:24:12.3076884Z   1 (5.00%) low severe
2026-08-02T22:24:12.3077101Z   2 (10.00%) low mild
2026-08-02T22:24:12.3080486Z Benchmarking full_cycle/4x4/10000key_1msg/String
2026-08-02T22:24:12.3081444Z Benchmarking full_cycle/4x4/10000key_1msg/String: Warming up for 500.00 ms
2026-08-02T22:24:12.9304818Z Benchmarking full_cycle/4x4/10000key_1msg/String: Collecting 20 samples in estimated 3.2114 s (160 iterations)
2026-08-02T22:24:16.1406649Z Benchmarking full_cycle/4x4/10000key_1msg/String: Analyzing
2026-08-02T22:24:16.2001017Z full_cycle/4x4/10000key_1msg/String
2026-08-02T22:24:16.2001923Z                         time:   [20.034 ms 20.060 ms 20.091 ms]
2026-08-02T22:24:16.2002333Z                         thrpt:  [14.435 Melem/s 14.457 Melem/s 14.475 Melem/s]
2026-08-02T22:24:16.2002887Z                  change:
2026-08-02T22:24:16.2003276Z                         time:   [+60.723% +61.057% +61.403%] (p = 0.00 < 0.05)
2026-08-02T22:24:16.2003901Z                         thrpt:  [−38.043% −37.910% −37.781%]
2026-08-02T22:24:16.2004271Z                         Performance has regressed.
2026-08-02T22:24:16.2004844Z Found 1 outliers among 20 measurements (5.00%)
2026-08-02T22:24:16.2005154Z   1 (5.00%) high mild
2026-08-02T22:24:16.2009060Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue
2026-08-02T22:24:16.2009535Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue: Warming up for 500.00 ms
2026-08-02T22:24:17.0566765Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue: Collecting 20 samples in estimated 3.2594 s (240 iterations)
2026-08-02T22:24:20.3031690Z Benchmarking full_cycle/4x4/10000key_1msg/DropValue: Analyzing
2026-08-02T22:24:20.3607074Z full_cycle/4x4/10000key_1msg/DropValue
2026-08-02T22:24:20.3607653Z                         time:   [13.509 ms 13.524 ms 13.537 ms]
2026-08-02T22:24:20.3608556Z                         thrpt:  [21.423 Melem/s 21.443 Melem/s 21.467 Melem/s]
2026-08-02T22:24:20.3609123Z                  change:
2026-08-02T22:24:20.3609670Z                         time:   [+54.521% +54.734% +54.910%] (p = 0.00 < 0.05)
2026-08-02T22:24:20.3610701Z                         thrpt:  [−35.446% −35.373% −35.284%]
2026-08-02T22:24:20.3611036Z                         Performance has regressed.
2026-08-02T22:24:20.3611793Z Found 2 outliers among 20 measurements (10.00%)
2026-08-02T22:24:20.3612314Z   1 (5.00%) low severe
2026-08-02T22:24:20.3612566Z   1 (5.00%) low mild
2026-08-02T22:24:20.3613105Z 
2026-08-02T22:24:20.3614520Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10000key_1msg/DropValue,19.000,24.000,40.100,443
2026-08-02T22:24:20.3617033Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10000key_1msg/DropValue,216.778,220.344,228.445,443
2026-08-02T22:24:20.3619994Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10000key_1msg/DropValue,30.718,31.417,32.902,443
2026-08-02T22:24:20.3622394Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10000key_1msg/DropValue,24.357,24.796,26.210,443
2026-08-02T22:24:20.3624655Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/10000key_1msg/DropValue,61.082,62.583,65.306,443
2026-08-02T22:24:20.3626305Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10000key_1msg/DropValue,69.276,71.266,74.210,443
2026-08-02T22:24:20.3628481Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10000key_1msg/DropValue,57.790,59.585,62.211,443
2026-08-02T22:24:20.3630312Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/10000key_1msg/DropValue,59.020,60.696,63.219,443
2026-08-02T22:24:20.3632501Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10000key_1msg/DropValue,194.477,196.602,204.185,443
2026-08-02T22:24:20.3633974Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10000key_1msg/String,19.000,26.000,39.100,303
2026-08-02T22:24:20.3635944Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10000key_1msg/String,216.943,220.378,226.675,303
2026-08-02T22:24:20.3637362Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10000key_1msg/String,54.366,56.437,58.505,303
2026-08-02T22:24:20.3639324Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10000key_1msg/String,92.831,95.612,97.789,303
2026-08-02T22:24:20.3640834Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/10000key_1msg/String,128.170,138.917,142.983,303
2026-08-02T22:24:20.3643050Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10000key_1msg/String,134.691,146.229,149.444,303
2026-08-02T22:24:20.3644623Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10000key_1msg/String,125.256,137.404,140.625,303
2026-08-02T22:24:20.3646681Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/10000key_1msg/String,128.165,139.562,148.786,303
2026-08-02T22:24:20.3648440Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10000key_1msg/String,195.490,197.924,206.397,303
2026-08-02T22:24:20.3650363Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10000key_1msg/u64,20.000,24.000,39.000,443
2026-08-02T22:24:20.3652240Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10000key_1msg/u64,211.899,216.087,246.653,443
2026-08-02T22:24:20.3654829Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10000key_1msg/u64,28.577,29.636,34.602,443
2026-08-02T22:24:20.3657285Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10000key_1msg/u64,23.768,24.170,26.845,443
2026-08-02T22:24:20.3660155Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/10000key_1msg/u64,60.793,62.250,73.426,443
2026-08-02T22:24:20.3661853Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10000key_1msg/u64,68.978,70.645,83.752,443
2026-08-02T22:24:20.3664002Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10000key_1msg/u64,57.540,59.822,67.931,443
2026-08-02T22:24:20.3665718Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/10000key_1msg/u64,58.558,59.945,66.853,443
2026-08-02T22:24:20.3667755Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10000key_1msg/u64,191.824,193.762,208.553,443
2026-08-02T22:24:20.3669188Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1000key_10msg/DropValue,19.000,23.100,32.100,887
2026-08-02T22:24:20.3671138Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1000key_10msg/DropValue,216.053,221.402,235.138,887
2026-08-02T22:24:20.3672680Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1000key_10msg/DropValue,28.111,29.088,29.982,887
2026-08-02T22:24:20.3674677Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1000key_10msg/DropValue,23.358,23.751,24.880,887
2026-08-02T22:24:20.3676238Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/1000key_10msg/DropValue,50.319,52.642,54.656,887
2026-08-02T22:24:20.3678379Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1000key_10msg/DropValue,56.155,58.194,60.766,887
2026-08-02T22:24:20.3679956Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1000key_10msg/DropValue,44.297,45.957,47.836,887
2026-08-02T22:24:20.3682182Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/1000key_10msg/DropValue,50.810,52.823,55.396,887
2026-08-02T22:24:20.3683731Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1000key_10msg/DropValue,217.475,221.523,234.307,887
2026-08-02T22:24:20.3685718Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1000key_10msg/String,19.000,25.000,39.100,383
2026-08-02T22:24:20.3687122Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1000key_10msg/String,217.796,223.707,253.001,383
2026-08-02T22:24:20.3689247Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1000key_10msg/String,54.515,65.312,68.247,383
2026-08-02T22:24:20.3690721Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1000key_10msg/String,92.606,94.996,99.136,383
2026-08-02T22:24:20.3693006Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/1000key_10msg/String,118.484,130.747,137.059,383
2026-08-02T22:24:20.3694604Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1000key_10msg/String,122.443,134.028,139.435,383
2026-08-02T22:24:20.3696688Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1000key_10msg/String,114.217,125.639,131.198,383
2026-08-02T22:24:20.3703060Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/1000key_10msg/String,120.564,132.551,136.346,383
2026-08-02T22:24:20.3704724Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1000key_10msg/String,216.774,221.883,238.424,383
2026-08-02T22:24:20.3706138Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1000key_10msg/u64,19.000,22.000,29.000,1055
2026-08-02T22:24:20.3707470Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1000key_10msg/u64,210.291,214.710,228.696,1055
2026-08-02T22:24:20.3708799Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1000key_10msg/u64,26.058,26.893,27.511,1055
2026-08-02T22:24:20.3710171Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1000key_10msg/u64,22.677,23.207,24.608,1055
2026-08-02T22:24:20.3712064Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/1000key_10msg/u64,48.611,50.393,52.572,1055
2026-08-02T22:24:20.3714391Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1000key_10msg/u64,54.779,56.697,58.151,1055
2026-08-02T22:24:20.3716466Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1000key_10msg/u64,43.682,45.371,46.919,1055
2026-08-02T22:24:20.3719005Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/1000key_10msg/u64,48.147,50.027,51.940,1055
2026-08-02T22:24:20.3721523Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1000key_10msg/u64,214.319,217.385,229.546,1055
2026-08-02T22:24:20.3724001Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/100key_100msg/DropValue,18.000,23.000,30.000,1175
2026-08-02T22:24:20.3726592Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/100key_100msg/DropValue,285.230,296.150,369.390,1175
2026-08-02T22:24:20.3729028Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/100key_100msg/DropValue,27.527,28.546,29.136,1175
2026-08-02T22:24:20.3731798Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/100key_100msg/DropValue,22.931,23.592,24.913,1175
2026-08-02T22:24:20.3734034Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/100key_100msg/DropValue,43.279,44.648,46.072,1175
2026-08-02T22:24:20.3736108Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/100key_100msg/DropValue,50.716,52.371,53.565,1175
2026-08-02T22:24:20.3738185Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/100key_100msg/DropValue,39.498,40.645,42.097,1175
2026-08-02T22:24:20.3740420Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/100key_100msg/DropValue,43.122,44.385,45.792,1175
2026-08-02T22:24:20.3742254Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/100key_100msg/DropValue,434.110,449.040,545.610,1175
2026-08-02T22:24:20.3744538Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/100key_100msg/String,19.100,24.100,32.000,443
2026-08-02T22:24:20.3746875Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/100key_100msg/String,314.290,322.810,387.320,443
2026-08-02T22:24:20.3749284Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/100key_100msg/String,53.211,64.049,67.784,443
2026-08-02T22:24:20.3751628Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/100key_100msg/String,94.286,98.505,102.538,443
2026-08-02T22:24:20.3754193Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/100key_100msg/String,112.350,123.751,126.170,443
2026-08-02T22:24:20.3756679Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/100key_100msg/String,117.384,128.826,131.019,443
2026-08-02T22:24:20.3759151Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/100key_100msg/String,110.513,122.076,125.145,443
2026-08-02T22:24:20.3760658Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/100key_100msg/String,113.099,124.802,127.615,443
2026-08-02T22:24:20.3762297Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/100key_100msg/String,451.640,464.560,563.650,443
2026-08-02T22:24:20.3764022Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/100key_100msg/u64,20.000,21.100,29.000,1195
2026-08-02T22:24:20.3766263Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/100key_100msg/u64,202.080,210.290,227.230,1195
2026-08-02T22:24:20.3768217Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/100key_100msg/u64,25.055,25.990,26.669,1195
2026-08-02T22:24:20.3769532Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/100key_100msg/u64,22.439,22.888,24.184,1195
2026-08-02T22:24:20.3770904Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/100key_100msg/u64,43.246,44.926,46.892,1195
2026-08-02T22:24:20.3772623Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/100key_100msg/u64,50.339,52.000,53.352,1195
2026-08-02T22:24:20.3774030Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/100key_100msg/u64,39.782,41.182,42.843,1195
2026-08-02T22:24:20.3775550Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/100key_100msg/u64,42.881,44.536,46.260,1195
2026-08-02T22:24:20.3776887Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/100key_100msg/u64,430.410,445.130,542.010,1195
2026-08-02T22:24:20.3778162Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10key_1000msg/DropValue,19.000,23.100,30.100,1235
2026-08-02T22:24:20.3779448Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10key_1000msg/DropValue,727.400,846.600,1227.200,1235
2026-08-02T22:24:20.3780742Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10key_1000msg/DropValue,27.189,28.279,28.722,1235
2026-08-02T22:24:20.3782288Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10key_1000msg/DropValue,22.533,23.624,24.977,1235
2026-08-02T22:24:20.3783697Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/10key_1000msg/DropValue,38.964,39.712,41.220,1235
2026-08-02T22:24:20.3785173Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10key_1000msg/DropValue,47.173,48.416,49.693,1235
2026-08-02T22:24:20.3786633Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10key_1000msg/DropValue,37.471,38.230,40.137,1235
2026-08-02T22:24:20.3788076Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/10key_1000msg/DropValue,39.044,39.819,41.163,1235
2026-08-02T22:24:20.3789482Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10key_1000msg/DropValue,1708.200,1767.300,1941.600,1235
2026-08-02T22:24:20.3790799Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10key_1000msg/String,19.000,29.000,40.100,443
2026-08-02T22:24:20.3792298Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10key_1000msg/String,987.800,1479.700,1801.400,443
2026-08-02T22:24:20.3793693Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10key_1000msg/String,52.704,60.888,66.808,443
2026-08-02T22:24:20.3794980Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10key_1000msg/String,92.697,95.935,100.596,443
2026-08-02T22:24:20.3796362Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/10key_1000msg/String,107.388,119.134,122.024,443
2026-08-02T22:24:20.3797804Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10key_1000msg/String,112.122,123.334,125.989,443
2026-08-02T22:24:20.3799248Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10key_1000msg/String,108.147,120.027,124.143,443
2026-08-02T22:24:20.3800763Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/10key_1000msg/String,108.720,120.321,123.959,443
2026-08-02T22:24:20.3802385Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10key_1000msg/String,1701.200,1806.400,2280.200,443
2026-08-02T22:24:20.3803659Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/10key_1000msg/u64,19.000,22.000,35.100,1275
2026-08-02T22:24:20.3804878Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/10key_1000msg/u64,201.400,226.400,285.600,1275
2026-08-02T22:24:20.3806103Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/10key_1000msg/u64,24.712,25.647,26.265,1275
2026-08-02T22:24:20.3807361Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/10key_1000msg/u64,21.902,22.324,23.815,1275
2026-08-02T22:24:20.3808700Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/10key_1000msg/u64,39.204,40.005,41.296,1275
2026-08-02T22:24:20.3810092Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/10key_1000msg/u64,46.093,47.266,49.089,1275
2026-08-02T22:24:20.3811927Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/10key_1000msg/u64,37.448,38.425,39.888,1275
2026-08-02T22:24:20.3813367Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/10key_1000msg/u64,38.953,40.101,41.384,1275
2026-08-02T22:24:20.3814708Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/10key_1000msg/u64,1708.200,1760.200,1956.700,1275
2026-08-02T22:24:20.3816011Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1key_10000msg/DropValue,19.000,22.100,31.000,1255
2026-08-02T22:24:20.3817457Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1key_10000msg/DropValue,8325.000,12133.000,16791.000,1255
2026-08-02T22:24:20.3818761Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1key_10000msg/DropValue,27.151,28.235,28.667,1255
2026-08-02T22:24:20.3820080Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1key_10000msg/DropValue,22.321,22.941,24.454,1255
2026-08-02T22:24:20.3821851Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/1key_10000msg/DropValue,38.311,39.026,40.239,1255
2026-08-02T22:24:20.3823325Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1key_10000msg/DropValue,46.789,47.955,49.089,1255
2026-08-02T22:24:20.3824927Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1key_10000msg/DropValue,36.846,37.694,39.057,1255
2026-08-02T22:24:20.3826395Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/1key_10000msg/DropValue,38.395,39.110,40.489,1255
2026-08-02T22:24:20.3827820Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1key_10000msg/DropValue,23034.000,23754.000,32361.000,1255
2026-08-02T22:24:20.3829144Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1key_10000msg/String,19.000,24.100,31.100,443
2026-08-02T22:24:20.3830422Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1key_10000msg/String,12162.000,15228.000,23494.000,443
2026-08-02T22:24:20.3831932Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1key_10000msg/String,51.663,60.011,65.952,443
2026-08-02T22:24:20.3833220Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1key_10000msg/String,91.318,94.125,96.736,443
2026-08-02T22:24:20.3834591Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/1key_10000msg/String,107.252,118.389,120.951,443
2026-08-02T22:24:20.3836024Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1key_10000msg/String,110.923,122.616,125.431,443
2026-08-02T22:24:20.3837463Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1key_10000msg/String,108.316,119.806,123.237,443
2026-08-02T22:24:20.3838879Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/1key_10000msg/String,108.813,119.889,122.571,443
2026-08-02T22:24:20.3840252Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1key_10000msg/String,21800.000,22342.000,31389.000,443
2026-08-02T22:24:20.3841738Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,16x1/1key_10000msg/u64,20.000,21.100,27.100,1295
2026-08-02T22:24:20.3843100Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,16x1/1key_10000msg/u64,260.000,320.000,591.000,1295
2026-08-02T22:24:20.3844336Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,16x1/1key_10000msg/u64,24.685,25.596,26.170,1295
2026-08-02T22:24:20.3845597Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,16x1/1key_10000msg/u64,21.644,22.371,23.702,1295
2026-08-02T22:24:20.3846932Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,16x1/1key_10000msg/u64,38.880,39.755,40.991,1295
2026-08-02T22:24:20.3848351Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,16x1/1key_10000msg/u64,45.895,46.781,47.861,1295
2026-08-02T22:24:20.3849866Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,16x1/1key_10000msg/u64,37.187,38.351,39.946,1295
2026-08-02T22:24:20.3851429Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,16x1/1key_10000msg/u64,38.388,39.754,41.307,1295
2026-08-02T22:24:20.3852779Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,16x1/1key_10000msg/u64,22742.000,23143.000,28523.000,1295
2026-08-02T22:24:20.3854079Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10000key_1msg/DropValue,18.000,24.000,38.100,443
2026-08-02T22:24:20.3855362Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10000key_1msg/DropValue,215.649,219.750,234.633,443
2026-08-02T22:24:20.3856631Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10000key_1msg/DropValue,30.299,31.278,33.954,443
2026-08-02T22:24:20.3857931Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10000key_1msg/DropValue,24.233,24.802,26.711,443
2026-08-02T22:24:20.3859343Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/10000key_1msg/DropValue,59.864,61.534,66.173,443
2026-08-02T22:24:20.3860792Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10000key_1msg/DropValue,68.557,70.908,76.808,443
2026-08-02T22:24:20.3862432Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10000key_1msg/DropValue,56.972,59.294,62.731,443
2026-08-02T22:24:20.3863858Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/10000key_1msg/DropValue,57.693,59.579,62.799,443
2026-08-02T22:24:20.3865217Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10000key_1msg/DropValue,193.544,195.765,213.096,443
2026-08-02T22:24:20.3866497Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10000key_1msg/String,20.000,27.100,46.100,323
2026-08-02T22:24:20.3867870Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10000key_1msg/String,225.989,231.047,245.522,323
2026-08-02T22:24:20.3869133Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10000key_1msg/String,54.309,55.664,57.500,323
2026-08-02T22:24:20.3870402Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10000key_1msg/String,82.075,84.907,87.933,323
2026-08-02T22:24:20.3872002Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/10000key_1msg/String,121.674,123.946,130.502,323
2026-08-02T22:24:20.3873446Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10000key_1msg/String,128.167,133.337,141.567,323
2026-08-02T22:24:20.3875001Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10000key_1msg/String,119.489,123.511,130.905,323
2026-08-02T22:24:20.3876417Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/10000key_1msg/String,122.316,124.560,132.649,323
2026-08-02T22:24:20.3877769Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10000key_1msg/String,194.594,196.929,204.884,323
2026-08-02T22:24:20.3879015Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10000key_1msg/u64,20.000,23.100,38.000,443
2026-08-02T22:24:20.3880247Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10000key_1msg/u64,212.377,216.551,225.614,443
2026-08-02T22:24:20.3881657Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10000key_1msg/u64,28.452,29.515,30.898,443
2026-08-02T22:24:20.3882899Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10000key_1msg/u64,23.443,24.058,25.327,443
2026-08-02T22:24:20.3884225Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/10000key_1msg/u64,58.982,61.753,65.592,443
2026-08-02T22:24:20.3885607Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10000key_1msg/u64,68.658,70.581,73.570,443
2026-08-02T22:24:20.3886998Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10000key_1msg/u64,57.642,59.693,61.912,443
2026-08-02T22:24:20.3888356Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/10000key_1msg/u64,57.526,59.223,62.650,443
2026-08-02T22:24:20.3889663Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10000key_1msg/u64,191.274,193.324,199.686,443
2026-08-02T22:24:20.3890938Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1000key_10msg/DropValue,18.000,23.000,31.000,1035
2026-08-02T22:24:20.3892522Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1000key_10msg/DropValue,212.977,220.340,233.275,1035
2026-08-02T22:24:20.3893799Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1000key_10msg/DropValue,27.952,29.062,29.815,1035
2026-08-02T22:24:20.3895097Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1000key_10msg/DropValue,22.889,23.560,24.692,1035
2026-08-02T22:24:20.3896491Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/1000key_10msg/DropValue,49.086,52.034,54.381,1035
2026-08-02T22:24:20.3897936Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1000key_10msg/DropValue,55.445,57.752,59.361,1035
2026-08-02T22:24:20.3899492Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1000key_10msg/DropValue,43.545,45.125,47.122,1035
2026-08-02T22:24:20.3900916Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/1000key_10msg/DropValue,49.227,51.827,54.007,1035
2026-08-02T22:24:20.3902483Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1000key_10msg/DropValue,215.521,221.433,233.896,1035
2026-08-02T22:24:20.3903769Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1000key_10msg/String,20.000,25.000,34.100,423
2026-08-02T22:24:20.3905021Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1000key_10msg/String,308.825,318.623,332.249,423
2026-08-02T22:24:20.3906252Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1000key_10msg/String,56.288,67.630,70.109,423
2026-08-02T22:24:20.3907533Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1000key_10msg/String,82.077,84.984,88.227,423
2026-08-02T22:24:20.3908900Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/1000key_10msg/String,116.321,121.449,127.157,423
2026-08-02T22:24:20.3910340Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1000key_10msg/String,120.819,124.710,130.868,423
2026-08-02T22:24:20.3912031Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1000key_10msg/String,111.346,115.309,120.719,423
2026-08-02T22:24:20.3913448Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/1000key_10msg/String,118.825,123.320,129.747,423
2026-08-02T22:24:20.3914793Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1000key_10msg/String,215.852,221.452,234.166,423
2026-08-02T22:24:20.3916048Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1000key_10msg/u64,19.000,21.100,28.100,1055
2026-08-02T22:24:20.3917398Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1000key_10msg/u64,210.211,214.389,228.566,1055
2026-08-02T22:24:20.3918605Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1000key_10msg/u64,26.075,26.808,27.471,1055
2026-08-02T22:24:20.3919842Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1000key_10msg/u64,22.042,23.386,25.350,1055
2026-08-02T22:24:20.3921175Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/1000key_10msg/u64,47.263,50.039,51.859,1055
2026-08-02T22:24:20.3922793Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1000key_10msg/u64,53.953,56.024,57.800,1055
2026-08-02T22:24:20.3924300Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1000key_10msg/u64,42.960,44.777,46.843,1055
2026-08-02T22:24:20.3925671Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/1000key_10msg/u64,46.547,49.157,51.323,1055
2026-08-02T22:24:20.3926985Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1000key_10msg/u64,214.149,217.826,230.049,1055
2026-08-02T22:24:20.3928266Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/100key_100msg/DropValue,18.000,22.100,30.000,1175
2026-08-02T22:24:20.3929545Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/100key_100msg/DropValue,274.010,285.830,340.340,1175
2026-08-02T22:24:20.3930831Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/100key_100msg/DropValue,27.460,28.510,29.203,1175
2026-08-02T22:24:20.3933364Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/100key_100msg/DropValue,22.596,23.434,24.713,1175
2026-08-02T22:24:20.3934869Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/100key_100msg/DropValue,43.381,44.605,45.896,1175
2026-08-02T22:24:20.3936327Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/100key_100msg/DropValue,50.294,51.785,53.021,1175
2026-08-02T22:24:20.3937771Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/100key_100msg/DropValue,39.750,40.712,42.091,1175
2026-08-02T22:24:20.3939198Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/100key_100msg/DropValue,43.208,44.366,45.784,1175
2026-08-02T22:24:20.3940560Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/100key_100msg/DropValue,434.210,449.840,546.020,1175
2026-08-02T22:24:20.3942039Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/100key_100msg/String,19.000,25.000,33.100,507
2026-08-02T22:24:20.3943426Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/100key_100msg/String,1212.460,1264.260,1391.490,507
2026-08-02T22:24:20.3944684Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/100key_100msg/String,52.994,63.715,68.569,507
2026-08-02T22:24:20.3945952Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/100key_100msg/String,82.809,87.759,91.827,507
2026-08-02T22:24:20.3947318Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/100key_100msg/String,107.656,112.193,115.267,507
2026-08-02T22:24:20.3948780Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/100key_100msg/String,112.765,117.997,121.224,507
2026-08-02T22:24:20.3950377Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/100key_100msg/String,105.936,110.727,113.796,507
2026-08-02T22:24:20.3952008Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/100key_100msg/String,108.304,113.532,116.035,507
2026-08-02T22:24:20.3953372Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/100key_100msg/String,447.030,463.760,567.960,507
2026-08-02T22:24:20.3954628Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/100key_100msg/u64,19.000,21.100,29.000,1215
2026-08-02T22:24:20.3955861Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/100key_100msg/u64,191.050,198.270,211.600,1215
2026-08-02T22:24:20.3957082Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/100key_100msg/u64,24.997,25.855,26.578,1215
2026-08-02T22:24:20.3958318Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/100key_100msg/u64,22.228,22.982,24.384,1215
2026-08-02T22:24:20.3959650Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/100key_100msg/u64,43.065,44.465,45.892,1215
2026-08-02T22:24:20.3961038Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/100key_100msg/u64,49.604,51.249,52.736,1215
2026-08-02T22:24:20.3962671Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/100key_100msg/u64,39.800,41.129,42.714,1215
2026-08-02T22:24:20.3964045Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/100key_100msg/u64,42.613,44.112,45.666,1215
2026-08-02T22:24:20.3965366Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/100key_100msg/u64,425.790,442.820,540.310,1215
2026-08-02T22:24:20.3966642Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10key_1000msg/DropValue,18.000,22.100,30.100,1255
2026-08-02T22:24:20.3968116Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10key_1000msg/DropValue,704.300,786.500,1081.000,1255
2026-08-02T22:24:20.3969419Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10key_1000msg/DropValue,27.169,28.210,28.688,1255
2026-08-02T22:24:20.3970725Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10key_1000msg/DropValue,21.952,23.025,24.798,1255
2026-08-02T22:24:20.3972343Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/10key_1000msg/DropValue,38.463,39.198,40.970,1255
2026-08-02T22:24:20.3973793Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10key_1000msg/DropValue,46.836,48.020,49.319,1255
2026-08-02T22:24:20.3975360Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10key_1000msg/DropValue,36.922,37.754,39.385,1255
2026-08-02T22:24:20.3976797Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/10key_1000msg/DropValue,38.503,39.234,40.968,1255
2026-08-02T22:24:20.3978183Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10key_1000msg/DropValue,1723.300,1771.300,1920.600,1255
2026-08-02T22:24:20.3979709Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10key_1000msg/String,19.000,25.000,38.000,527
2026-08-02T22:24:20.3980999Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10key_1000msg/String,1072.000,10898.300,12462.200,527
2026-08-02T22:24:20.3982445Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10key_1000msg/String,54.121,63.441,69.821,527
2026-08-02T22:24:20.3983717Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10key_1000msg/String,81.093,85.153,88.959,527
2026-08-02T22:24:20.3985081Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/10key_1000msg/String,103.647,108.371,111.426,527
2026-08-02T22:24:20.3986507Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10key_1000msg/String,107.600,112.764,115.936,527
2026-08-02T22:24:20.3987942Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10key_1000msg/String,103.466,108.867,112.725,527
2026-08-02T22:24:20.3989350Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/10key_1000msg/String,104.262,109.211,111.987,527
2026-08-02T22:24:20.3990711Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10key_1000msg/String,1701.200,1877.500,11698.800,527
2026-08-02T22:24:20.3992198Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/10key_1000msg/u64,20.000,21.100,29.100,1275
2026-08-02T22:24:20.3993579Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/10key_1000msg/u64,202.400,225.500,280.500,1275
2026-08-02T22:24:20.3994805Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/10key_1000msg/u64,24.671,25.628,26.114,1275
2026-08-02T22:24:20.3996059Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/10key_1000msg/u64,21.503,22.640,23.948,1275
2026-08-02T22:24:20.3997395Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/10key_1000msg/u64,39.735,41.075,42.423,1275
2026-08-02T22:24:20.3998793Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/10key_1000msg/u64,46.796,47.998,49.241,1275
2026-08-02T22:24:20.4000290Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/10key_1000msg/u64,38.084,39.217,40.776,1275
2026-08-02T22:24:20.4001865Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/10key_1000msg/u64,39.353,40.697,42.096,1275
2026-08-02T22:24:20.4003204Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/10key_1000msg/u64,1678.100,1735.200,2246.200,1275
2026-08-02T22:24:20.4004485Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1key_10000msg/DropValue,19.000,22.100,30.100,1235
2026-08-02T22:24:20.4005791Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1key_10000msg/DropValue,8486.000,12973.000,17522.000,1235
2026-08-02T22:24:20.4007074Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1key_10000msg/DropValue,27.109,28.268,28.664,1235
2026-08-02T22:24:20.4008382Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1key_10000msg/DropValue,21.936,22.721,24.966,1235
2026-08-02T22:24:20.4009767Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/1key_10000msg/DropValue,38.228,39.433,44.312,1235
2026-08-02T22:24:20.4011215Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1key_10000msg/DropValue,46.652,48.066,56.061,1235
2026-08-02T22:24:20.4012856Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1key_10000msg/DropValue,36.770,37.346,38.779,1235
2026-08-02T22:24:20.4014284Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/1key_10000msg/DropValue,38.349,39.597,42.660,1235
2026-08-02T22:24:20.4015673Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1key_10000msg/DropValue,23123.000,24074.000,30306.000,1235
2026-08-02T22:24:20.4017097Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1key_10000msg/String,19.000,23.100,35.000,527
2026-08-02T22:24:20.4018373Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1key_10000msg/String,129431.000,135122.000,147855.000,527
2026-08-02T22:24:20.4019641Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1key_10000msg/String,51.304,61.497,67.766,527
2026-08-02T22:24:20.4020906Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1key_10000msg/String,80.507,83.098,86.421,527
2026-08-02T22:24:20.4022527Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/1key_10000msg/String,101.800,102.638,110.745,527
2026-08-02T22:24:20.4024091Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1key_10000msg/String,105.567,106.963,113.734,527
2026-08-02T22:24:20.4025520Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1key_10000msg/String,101.641,102.587,111.385,527
2026-08-02T22:24:20.4026928Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/1key_10000msg/String,101.175,102.972,112.336,527
2026-08-02T22:24:20.4028290Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1key_10000msg/String,21901.000,23424.000,30677.000,527
2026-08-02T22:24:20.4029568Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x1/1key_10000msg/u64,20.000,21.100,27.000,1295
2026-08-02T22:24:20.4030787Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x1/1key_10000msg/u64,250.000,291.000,581.000,1295
2026-08-02T22:24:20.4032240Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x1/1key_10000msg/u64,24.595,25.616,26.021,1295
2026-08-02T22:24:20.4033480Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x1/1key_10000msg/u64,21.480,22.486,23.639,1295
2026-08-02T22:24:20.4034811Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x1/1key_10000msg/u64,38.734,39.977,41.912,1295
2026-08-02T22:24:20.4036209Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x1/1key_10000msg/u64,45.786,46.699,47.708,1295
2026-08-02T22:24:20.4037599Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x1/1key_10000msg/u64,37.079,38.697,40.706,1295
2026-08-02T22:24:20.4038967Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x1/1key_10000msg/u64,38.456,40.150,41.800,1295
2026-08-02T22:24:20.4040307Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x1/1key_10000msg/u64,22822.000,23244.000,26099.000,1295
2026-08-02T22:24:20.4041958Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10000key_1msg/DropValue,20.100,43.100,71.100,115
2026-08-02T22:24:20.4043261Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10000key_1msg/DropValue,41.776,42.136,42.929,115
2026-08-02T22:24:20.4044521Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10000key_1msg/DropValue,28.723,29.350,29.474,115
2026-08-02T22:24:20.4045825Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10000key_1msg/DropValue,24.478,25.506,27.472,115
2026-08-02T22:24:20.4047216Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/10000key_1msg/DropValue,59.616,62.627,72.882,115
2026-08-02T22:24:20.4048802Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10000key_1msg/DropValue,66.028,68.212,76.607,115
2026-08-02T22:24:20.4050255Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10000key_1msg/DropValue,54.909,56.713,74.118,115
2026-08-02T22:24:20.4051885Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/10000key_1msg/DropValue,58.996,61.666,78.596,115
2026-08-02T22:24:20.4053252Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10000key_1msg/DropValue,38.035,38.318,39.080,115
2026-08-02T22:24:20.4054529Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10000key_1msg/String,31.100,45.100,76.100,95
2026-08-02T22:24:20.4055771Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10000key_1msg/String,42.843,43.444,44.440,95
2026-08-02T22:24:20.4056998Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10000key_1msg/String,46.627,47.128,47.445,95
2026-08-02T22:24:20.4058264Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10000key_1msg/String,82.969,85.656,89.081,95
2026-08-02T22:24:20.4059630Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/10000key_1msg/String,121.900,127.725,144.083,95
2026-08-02T22:24:20.4061070Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10000key_1msg/String,127.608,134.060,163.496,95
2026-08-02T22:24:20.4062682Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10000key_1msg/String,118.443,122.681,150.083,95
2026-08-02T22:24:20.4064092Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/10000key_1msg/String,122.342,129.418,147.003,95
2026-08-02T22:24:20.4065426Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10000key_1msg/String,38.025,38.377,39.291,95
2026-08-02T22:24:20.4066813Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10000key_1msg/u64,26.000,41.000,57.100,115
2026-08-02T22:24:20.4068040Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10000key_1msg/u64,41.644,43.673,45.312,115
2026-08-02T22:24:20.4069236Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10000key_1msg/u64,27.142,27.967,28.172,115
2026-08-02T22:24:20.4070482Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10000key_1msg/u64,23.827,24.761,26.649,115
2026-08-02T22:24:20.4072046Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/10000key_1msg/u64,59.904,62.939,84.103,115
2026-08-02T22:24:20.4073584Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10000key_1msg/u64,66.376,68.881,92.096,115
2026-08-02T22:24:20.4074967Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10000key_1msg/u64,54.680,56.897,69.312,115
2026-08-02T22:24:20.4076352Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/10000key_1msg/u64,58.620,61.688,74.331,115
2026-08-02T22:24:20.4077666Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10000key_1msg/u64,37.965,48.612,50.422,115
2026-08-02T22:24:20.4078938Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1000key_10msg/DropValue,19.000,28.000,55.100,171
2026-08-02T22:24:20.4080207Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1000key_10msg/DropValue,41.234,42.135,42.674,171
2026-08-02T22:24:20.4081686Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1000key_10msg/DropValue,27.066,27.940,28.045,171
2026-08-02T22:24:20.4083003Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1000key_10msg/DropValue,22.917,24.548,25.822,171
2026-08-02T22:24:20.4084396Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/1000key_10msg/DropValue,51.302,54.158,56.744,171
2026-08-02T22:24:20.4085859Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1000key_10msg/DropValue,56.508,58.799,60.848,171
2026-08-02T22:24:20.4087321Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1000key_10msg/DropValue,44.363,46.310,49.146,171
2026-08-02T22:24:20.4088757Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/1000key_10msg/DropValue,51.209,54.296,56.394,171
2026-08-02T22:24:20.4090124Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1000key_10msg/DropValue,39.282,40.250,41.098,171
2026-08-02T22:24:20.4091792Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1000key_10msg/String,22.100,31.100,49.100,115
2026-08-02T22:24:20.4093074Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1000key_10msg/String,47.357,48.367,49.629,115
2026-08-02T22:24:20.4094308Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1000key_10msg/String,44.514,44.873,45.275,115
2026-08-02T22:24:20.4095612Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1000key_10msg/String,82.055,84.332,86.602,115
2026-08-02T22:24:20.4097007Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/1000key_10msg/String,118.418,122.477,129.020,115
2026-08-02T22:24:20.4098569Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1000key_10msg/String,121.207,124.541,131.361,115
2026-08-02T22:24:20.4100010Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1000key_10msg/String,112.242,115.725,121.358,115
2026-08-02T22:24:20.4101589Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/1000key_10msg/String,120.440,124.181,130.338,115
2026-08-02T22:24:20.4102946Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1000key_10msg/String,39.256,40.131,41.735,115
2026-08-02T22:24:20.4104220Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1000key_10msg/u64,20.000,26.100,40.000,191
2026-08-02T22:24:20.4105442Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1000key_10msg/u64,40.882,41.802,42.691,191
2026-08-02T22:24:20.4106647Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1000key_10msg/u64,25.090,25.633,25.798,191
2026-08-02T22:24:20.4107896Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1000key_10msg/u64,22.611,23.218,24.522,191
2026-08-02T22:24:20.4109235Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/1000key_10msg/u64,49.636,51.835,53.415,191
2026-08-02T22:24:20.4110632Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1000key_10msg/u64,54.809,56.970,58.891,191
2026-08-02T22:24:20.4112127Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1000key_10msg/u64,42.956,44.760,47.524,191
2026-08-02T22:24:20.4113507Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/1000key_10msg/u64,48.819,50.794,53.292,191
2026-08-02T22:24:20.4114822Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1000key_10msg/u64,39.013,40.004,40.785,191
2026-08-02T22:24:20.4116210Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/100key_100msg/DropValue,19.000,27.100,53.100,191
2026-08-02T22:24:20.4117501Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/100key_100msg/DropValue,43.951,44.564,50.194,191
2026-08-02T22:24:20.4129299Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/100key_100msg/DropValue,26.815,27.584,27.725,191
2026-08-02T22:24:20.4130761Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/100key_100msg/DropValue,22.872,23.535,24.881,191
2026-08-02T22:24:20.4132399Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/100key_100msg/DropValue,43.971,45.268,46.636,191
2026-08-02T22:24:20.4134111Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/100key_100msg/DropValue,50.935,52.477,53.727,191
2026-08-02T22:24:20.4135591Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/100key_100msg/DropValue,39.990,41.247,42.801,191
2026-08-02T22:24:20.4137070Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/100key_100msg/DropValue,43.851,45.353,46.972,191
2026-08-02T22:24:20.4138460Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/100key_100msg/DropValue,56.136,58.146,65.678,191
2026-08-02T22:24:20.4139752Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/100key_100msg/String,23.000,32.100,49.100,115
2026-08-02T22:24:20.4141007Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/100key_100msg/String,44.839,45.535,107.519,115
2026-08-02T22:24:20.4142409Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/100key_100msg/String,44.105,44.625,45.169,115
2026-08-02T22:24:20.4143683Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/100key_100msg/String,83.430,87.402,91.040,115
2026-08-02T22:24:20.4145064Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/100key_100msg/String,108.356,111.319,114.383,115
2026-08-02T22:24:20.4146516Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/100key_100msg/String,113.514,116.564,119.652,115
2026-08-02T22:24:20.4147971Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/100key_100msg/String,106.268,109.833,116.284,115
2026-08-02T22:24:20.4149434Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/100key_100msg/String,110.204,112.595,116.256,115
2026-08-02T22:24:20.4150807Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/100key_100msg/String,55.854,121.063,129.961,115
2026-08-02T22:24:20.4152411Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/100key_100msg/u64,20.000,24.100,42.000,211
2026-08-02T22:24:20.4153651Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/100key_100msg/u64,39.868,40.814,46.568,211
2026-08-02T22:24:20.4154872Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/100key_100msg/u64,24.546,25.043,25.190,211
2026-08-02T22:24:20.4156125Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/100key_100msg/u64,21.632,22.898,23.960,211
2026-08-02T22:24:20.4157471Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/100key_100msg/u64,43.997,45.279,46.587,211
2026-08-02T22:24:20.4158980Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/100key_100msg/u64,50.475,51.969,53.392,211
2026-08-02T22:24:20.4160361Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/100key_100msg/u64,39.589,40.770,41.918,211
2026-08-02T22:24:20.4161855Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/100key_100msg/u64,44.444,45.541,47.098,211
2026-08-02T22:24:20.4163180Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/100key_100msg/u64,54.301,56.437,62.667,211
2026-08-02T22:24:20.4164461Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10key_1000msg/DropValue,19.000,26.000,46.000,191
2026-08-02T22:24:20.4165736Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10key_1000msg/DropValue,79.775,85.162,105.450,191
2026-08-02T22:24:20.4167008Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10key_1000msg/DropValue,27.534,28.265,28.479,191
2026-08-02T22:24:20.4168318Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10key_1000msg/DropValue,22.326,23.213,25.008,191
2026-08-02T22:24:20.4169707Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/10key_1000msg/DropValue,38.537,39.175,40.386,191
2026-08-02T22:24:20.4171157Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10key_1000msg/DropValue,46.857,47.946,48.847,191
2026-08-02T22:24:20.4172707Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10key_1000msg/DropValue,37.076,37.676,38.975,191
2026-08-02T22:24:20.4174126Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/10key_1000msg/DropValue,38.594,39.148,40.156,191
2026-08-02T22:24:20.4175503Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10key_1000msg/DropValue,133.625,137.881,155.287,191
2026-08-02T22:24:20.4176917Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10key_1000msg/String,21.100,33.000,53.200,115
2026-08-02T22:24:20.4178174Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10key_1000msg/String,675.506,720.462,791.100,115
2026-08-02T22:24:20.4179416Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10key_1000msg/String,43.989,44.784,45.255,115
2026-08-02T22:24:20.4180689Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10key_1000msg/String,81.038,84.184,87.295,115
2026-08-02T22:24:20.4182170Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/10key_1000msg/String,104.410,108.177,110.974,115
2026-08-02T22:24:20.4183723Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10key_1000msg/String,108.355,112.198,114.204,115
2026-08-02T22:24:20.4185155Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10key_1000msg/String,104.425,108.747,112.905,115
2026-08-02T22:24:20.4186580Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/10key_1000msg/String,104.969,108.783,111.555,115
2026-08-02T22:24:20.4187950Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10key_1000msg/String,139.256,151.094,201.750,115
2026-08-02T22:24:20.4189250Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/10key_1000msg/u64,20.000,25.100,46.100,211
2026-08-02T22:24:20.4190501Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/10key_1000msg/u64,41.956,44.894,55.606,211
2026-08-02T22:24:20.4191816Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/10key_1000msg/u64,24.647,25.266,25.406,211
2026-08-02T22:24:20.4193074Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/10key_1000msg/u64,21.753,22.659,23.727,211
2026-08-02T22:24:20.4194424Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/10key_1000msg/u64,39.095,39.868,41.020,211
2026-08-02T22:24:20.4195833Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/10key_1000msg/u64,46.093,47.019,47.782,211
2026-08-02T22:24:20.4197246Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/10key_1000msg/u64,37.456,37.997,39.179,211
2026-08-02T22:24:20.4198673Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/10key_1000msg/u64,38.883,39.744,40.724,211
2026-08-02T22:24:20.4200021Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/10key_1000msg/u64,133.125,139.075,154.475,211
2026-08-02T22:24:20.4201542Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1key_10000msg/DropValue,19.000,26.100,42.000,191
2026-08-02T22:24:20.4202855Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1key_10000msg/DropValue,617.375,775.188,1091.375,191
2026-08-02T22:24:20.4204154Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1key_10000msg/DropValue,27.673,28.402,28.578,191
2026-08-02T22:24:20.4205479Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1key_10000msg/DropValue,21.791,22.603,24.104,191
2026-08-02T22:24:20.4206898Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/1key_10000msg/DropValue,38.227,38.784,40.194,191
2026-08-02T22:24:20.4208500Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1key_10000msg/DropValue,46.716,47.821,48.706,191
2026-08-02T22:24:20.4209968Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1key_10000msg/DropValue,36.788,37.538,38.723,191
2026-08-02T22:24:20.4211526Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/1key_10000msg/DropValue,38.311,39.034,40.263,191
2026-08-02T22:24:20.4212932Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1key_10000msg/DropValue,1466.500,1515.938,2209.062,191
2026-08-02T22:24:20.4214262Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1key_10000msg/String,21.100,36.000,54.100,115
2026-08-02T22:24:20.4215552Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1key_10000msg/String,8199.000,8421.312,9009.312,115
2026-08-02T22:24:20.4216828Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1key_10000msg/String,44.181,45.021,45.606,115
2026-08-02T22:24:20.4218117Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1key_10000msg/String,80.648,83.278,86.105,115
2026-08-02T22:24:20.4219512Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/1key_10000msg/String,101.996,102.732,109.066,115
2026-08-02T22:24:20.4220970Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1key_10000msg/String,106.120,107.088,113.381,115
2026-08-02T22:24:20.4222535Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1key_10000msg/String,102.201,103.293,113.277,115
2026-08-02T22:24:20.4223967Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/1key_10000msg/String,102.307,103.223,110.505,115
2026-08-02T22:24:20.4225355Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1key_10000msg/String,1421.375,1516.562,2087.625,115
2026-08-02T22:24:20.4226778Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,1x16/1key_10000msg/u64,20.000,24.100,36.100,211
2026-08-02T22:24:20.4228012Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,1x16/1key_10000msg/u64,48.188,59.438,105.812,211
2026-08-02T22:24:20.4229231Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,1x16/1key_10000msg/u64,24.804,25.400,25.565,211
2026-08-02T22:24:20.4230491Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,1x16/1key_10000msg/u64,21.313,22.507,24.576,211
2026-08-02T22:24:20.4231956Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,1x16/1key_10000msg/u64,38.816,39.575,40.647,211
2026-08-02T22:24:20.4233481Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,1x16/1key_10000msg/u64,45.821,46.859,47.673,211
2026-08-02T22:24:20.4234881Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,1x16/1key_10000msg/u64,37.046,38.334,39.675,211
2026-08-02T22:24:20.4236282Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,1x16/1key_10000msg/u64,38.186,39.682,40.904,211
2026-08-02T22:24:20.4237622Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,1x16/1key_10000msg/u64,1457.688,1482.750,1545.375,211
2026-08-02T22:24:20.4238925Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10000key_1msg/DropValue,19.000,26.000,48.100,303
2026-08-02T22:24:20.4240209Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10000key_1msg/DropValue,78.283,79.447,82.281,303
2026-08-02T22:24:20.4241668Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10000key_1msg/DropValue,28.643,29.333,29.780,303
2026-08-02T22:24:20.4242986Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10000key_1msg/DropValue,24.126,24.604,25.728,303
2026-08-02T22:24:20.4244394Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/10000key_1msg/DropValue,60.072,63.083,66.183,303
2026-08-02T22:24:20.4245851Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10000key_1msg/DropValue,70.099,72.771,75.379,303
2026-08-02T22:24:20.4247307Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10000key_1msg/DropValue,55.305,57.973,59.998,303
2026-08-02T22:24:20.4248781Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/10000key_1msg/DropValue,59.183,63.026,66.062,303
2026-08-02T22:24:20.4250160Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10000key_1msg/DropValue,69.388,70.043,72.103,303
2026-08-02T22:24:20.4251677Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10000key_1msg/String,21.000,33.000,53.100,191
2026-08-02T22:24:20.4252933Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10000key_1msg/String,80.736,82.145,85.285,191
2026-08-02T22:24:20.4254169Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10000key_1msg/String,48.853,49.657,50.306,191
2026-08-02T22:24:20.4255449Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10000key_1msg/String,83.748,86.320,88.814,191
2026-08-02T22:24:20.4256844Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/10000key_1msg/String,122.683,125.691,132.044,191
2026-08-02T22:24:20.4258399Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10000key_1msg/String,127.397,132.053,138.867,191
2026-08-02T22:24:20.4259845Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10000key_1msg/String,117.011,121.838,127.863,191
2026-08-02T22:24:20.4261388Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/10000key_1msg/String,124.673,127.742,134.036,191
2026-08-02T22:24:20.4262750Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10000key_1msg/String,69.273,70.141,72.441,191
2026-08-02T22:24:20.4264017Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10000key_1msg/u64,20.000,25.000,45.100,303
2026-08-02T22:24:20.4265233Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10000key_1msg/u64,77.858,78.984,81.117,303
2026-08-02T22:24:20.4266443Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10000key_1msg/u64,27.024,27.848,28.274,303
2026-08-02T22:24:20.4267703Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10000key_1msg/u64,23.721,24.332,25.672,303
2026-08-02T22:24:20.4269044Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/10000key_1msg/u64,61.977,64.195,66.488,303
2026-08-02T22:24:20.4270457Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10000key_1msg/u64,70.025,72.733,74.498,303
2026-08-02T22:24:20.4271956Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10000key_1msg/u64,55.735,58.823,61.438,303
2026-08-02T22:24:20.4273348Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/10000key_1msg/u64,61.273,63.817,67.515,303
2026-08-02T22:24:20.4274664Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10000key_1msg/u64,68.623,69.184,70.904,303
2026-08-02T22:24:20.4276057Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1000key_10msg/DropValue,19.000,23.000,33.100,527
2026-08-02T22:24:20.4277353Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1000key_10msg/DropValue,77.367,78.982,82.982,527
2026-08-02T22:24:20.4278623Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1000key_10msg/DropValue,27.598,28.405,28.735,527
2026-08-02T22:24:20.4279929Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1000key_10msg/DropValue,23.391,24.834,26.149,527
2026-08-02T22:24:20.4281452Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/1000key_10msg/DropValue,49.831,52.264,54.323,527
2026-08-02T22:24:20.4283023Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1000key_10msg/DropValue,55.460,57.781,60.088,527
2026-08-02T22:24:20.4284474Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1000key_10msg/DropValue,43.473,45.259,47.720,527
2026-08-02T22:24:20.4285912Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/1000key_10msg/DropValue,50.227,52.712,54.836,527
2026-08-02T22:24:20.4287298Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1000key_10msg/DropValue,74.985,76.788,81.084,527
2026-08-02T22:24:20.4288586Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1000key_10msg/String,19.000,25.100,37.000,283
2026-08-02T22:24:20.4289851Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1000key_10msg/String,101.850,104.595,107.951,283
2026-08-02T22:24:20.4291097Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1000key_10msg/String,47.745,48.326,49.691,283
2026-08-02T22:24:20.4292492Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1000key_10msg/String,83.025,85.549,87.563,283
2026-08-02T22:24:20.4293882Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/1000key_10msg/String,117.336,121.064,126.619,283
2026-08-02T22:24:20.4295334Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1000key_10msg/String,120.223,124.077,131.035,283
2026-08-02T22:24:20.4296783Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1000key_10msg/String,111.326,115.584,120.510,283
2026-08-02T22:24:20.4298215Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/1000key_10msg/String,119.945,123.578,129.938,283
2026-08-02T22:24:20.4299590Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1000key_10msg/String,75.140,76.891,80.380,283
2026-08-02T22:24:20.4300976Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1000key_10msg/u64,20.000,21.100,30.000,567
2026-08-02T22:24:20.4302307Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1000key_10msg/u64,75.794,77.527,82.063,567
2026-08-02T22:24:20.4303520Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1000key_10msg/u64,25.437,25.983,26.442,567
2026-08-02T22:24:20.4304774Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1000key_10msg/u64,22.709,23.517,24.947,567
2026-08-02T22:24:20.4306121Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/1000key_10msg/u64,49.124,51.667,55.647,567
2026-08-02T22:24:20.4307628Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1000key_10msg/u64,55.026,57.151,58.914,567
2026-08-02T22:24:20.4309023Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1000key_10msg/u64,43.358,45.194,47.387,567
2026-08-02T22:24:20.4310409Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/1000key_10msg/u64,48.902,51.276,53.917,567
2026-08-02T22:24:20.4311840Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1000key_10msg/u64,74.429,75.897,79.896,567
2026-08-02T22:24:20.4313124Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/100key_100msg/DropValue,18.100,23.000,32.100,587
2026-08-02T22:24:20.4314401Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/100key_100msg/DropValue,86.838,88.540,107.150,587
2026-08-02T22:24:20.4315668Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/100key_100msg/DropValue,27.134,27.929,28.281,587
2026-08-02T22:24:20.4316977Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/100key_100msg/DropValue,22.909,23.799,25.741,587
2026-08-02T22:24:20.4318394Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/100key_100msg/DropValue,44.488,45.684,46.990,587
2026-08-02T22:24:20.4319863Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/100key_100msg/DropValue,51.551,53.218,54.959,587
2026-08-02T22:24:20.4321414Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/100key_100msg/DropValue,39.878,41.101,42.554,587
2026-08-02T22:24:20.4322856Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/100key_100msg/DropValue,44.389,45.657,47.077,587
2026-08-02T22:24:20.4324237Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/100key_100msg/DropValue,136.030,141.715,167.412,587
2026-08-02T22:24:20.4325660Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/100key_100msg/String,20.000,26.000,37.100,303
2026-08-02T22:24:20.4326911Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/100key_100msg/String,89.567,91.698,114.612,303
2026-08-02T22:24:20.4328161Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/100key_100msg/String,45.954,47.370,48.733,303
2026-08-02T22:24:20.4329443Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/100key_100msg/String,83.939,88.089,91.643,303
2026-08-02T22:24:20.4330836Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/100key_100msg/String,108.748,111.829,114.765,303
2026-08-02T22:24:20.4332613Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/100key_100msg/String,114.591,117.607,120.165,303
2026-08-02T22:24:20.4334057Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/100key_100msg/String,107.214,109.792,113.211,303
2026-08-02T22:24:20.4335486Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/100key_100msg/String,110.155,113.543,117.441,303
2026-08-02T22:24:20.4336856Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/100key_100msg/String,140.262,401.322,433.108,303
2026-08-02T22:24:20.4338128Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/100key_100msg/u64,19.100,22.000,30.000,627
2026-08-02T22:24:20.4339337Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/100key_100msg/u64,66.222,67.625,70.308,627
2026-08-02T22:24:20.4340534Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/100key_100msg/u64,24.737,25.303,25.633,627
2026-08-02T22:24:20.4341894Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/100key_100msg/u64,22.285,23.858,25.757,627
2026-08-02T22:24:20.4343270Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/100key_100msg/u64,44.466,45.978,47.593,627
2026-08-02T22:24:20.4344662Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/100key_100msg/u64,51.388,52.959,54.189,627
2026-08-02T22:24:20.4346057Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/100key_100msg/u64,40.125,41.409,42.758,627
2026-08-02T22:24:20.4347439Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/100key_100msg/u64,44.255,45.621,47.189,627
2026-08-02T22:24:20.4348789Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/100key_100msg/u64,135.178,139.835,166.060,627
2026-08-02T22:24:20.4350239Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10key_1000msg/DropValue,18.000,23.100,35.100,607
2026-08-02T22:24:20.4351676Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10key_1000msg/DropValue,203.375,218.675,321.325,607
2026-08-02T22:24:20.4352946Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10key_1000msg/DropValue,27.598,28.380,28.704,607
2026-08-02T22:24:20.4354250Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10key_1000msg/DropValue,22.265,23.266,24.894,607
2026-08-02T22:24:20.4355649Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/10key_1000msg/DropValue,38.496,39.169,40.875,607
2026-08-02T22:24:20.4357216Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10key_1000msg/DropValue,46.857,47.991,49.489,607
2026-08-02T22:24:20.4358659Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10key_1000msg/DropValue,37.048,37.870,39.650,607
2026-08-02T22:24:20.4360086Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/10key_1000msg/DropValue,38.626,39.270,41.057,607
2026-08-02T22:24:20.4361582Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10key_1000msg/DropValue,456.350,470.125,532.000,607
2026-08-02T22:24:20.4362885Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10key_1000msg/String,19.000,25.100,39.100,303
2026-08-02T22:24:20.4364142Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10key_1000msg/String,285.800,379.975,450.825,303
2026-08-02T22:24:20.4365388Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10key_1000msg/String,46.060,48.192,49.830,303
2026-08-02T22:24:20.4366654Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10key_1000msg/String,81.869,85.632,88.806,303
2026-08-02T22:24:20.4368033Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/10key_1000msg/String,104.139,108.221,110.356,303
2026-08-02T22:24:20.4369473Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10key_1000msg/String,108.194,112.412,114.781,303
2026-08-02T22:24:20.4370910Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10key_1000msg/String,104.443,108.720,111.153,303
2026-08-02T22:24:20.4372442Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/10key_1000msg/String,104.757,109.111,111.854,303
2026-08-02T22:24:20.4373946Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10key_1000msg/String,471.400,3070.725,3477.975,303
2026-08-02T22:24:20.4375221Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/10key_1000msg/u64,20.000,22.000,30.100,647
2026-08-02T22:24:20.4376433Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/10key_1000msg/u64,71.400,76.650,95.925,647
2026-08-02T22:24:20.4377635Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/10key_1000msg/u64,24.835,25.452,25.690,647
2026-08-02T22:24:20.4378883Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/10key_1000msg/u64,21.677,22.335,23.478,647
2026-08-02T22:24:20.4380220Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/10key_1000msg/u64,38.976,40.336,42.482,647
2026-08-02T22:24:20.4381828Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/10key_1000msg/u64,45.954,47.023,47.984,647
2026-08-02T22:24:20.4383224Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/10key_1000msg/u64,37.279,38.669,41.874,647
2026-08-02T22:24:20.4384598Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/10key_1000msg/u64,38.808,40.328,42.500,647
2026-08-02T22:24:20.4385918Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/10key_1000msg/u64,454.350,469.375,525.475,647
2026-08-02T22:24:20.4387199Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1key_10000msg/DropValue,18.000,24.000,34.100,607
2026-08-02T22:24:20.4388493Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1key_10000msg/DropValue,2194.000,2707.500,3854.500,607
2026-08-02T22:24:20.4389771Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1key_10000msg/DropValue,27.557,28.340,28.634,607
2026-08-02T22:24:20.4391081Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1key_10000msg/DropValue,22.085,22.688,24.100,607
2026-08-02T22:24:20.4392634Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/1key_10000msg/DropValue,38.218,38.507,40.465,607
2026-08-02T22:24:20.4394085Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1key_10000msg/DropValue,46.621,47.245,48.337,607
2026-08-02T22:24:20.4395542Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1key_10000msg/DropValue,36.757,37.131,38.866,607
2026-08-02T22:24:20.4396978Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/1key_10000msg/DropValue,38.295,38.706,39.941,607
2026-08-02T22:24:20.4398493Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1key_10000msg/DropValue,5758.250,5883.500,6537.250,607
2026-08-02T22:24:20.4399814Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1key_10000msg/String,19.000,25.000,32.100,303
2026-08-02T22:24:20.4401089Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1key_10000msg/String,32377.750,32908.750,35573.750,303
2026-08-02T22:24:20.4402477Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1key_10000msg/String,45.685,48.052,50.198,303
2026-08-02T22:24:20.4403764Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1key_10000msg/String,81.588,84.164,86.742,303
2026-08-02T22:24:20.4405159Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/1key_10000msg/String,101.735,102.515,109.071,303
2026-08-02T22:24:20.4406713Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1key_10000msg/String,105.598,106.614,113.094,303
2026-08-02T22:24:20.4408160Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1key_10000msg/String,102.023,102.782,110.332,303
2026-08-02T22:24:20.4409599Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/1key_10000msg/String,102.064,102.926,110.505,303
2026-08-02T22:24:20.4410973Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1key_10000msg/String,5525.250,5688.250,6457.000,303
2026-08-02T22:24:20.4412393Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,create,4x4/1key_10000msg/u64,20.000,22.000,29.100,647
2026-08-02T22:24:20.4413619Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,drop,4x4/1key_10000msg/u64,92.750,112.750,208.000,647
2026-08-02T22:24:20.4414841Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,recv,4x4/1key_10000msg/u64,24.712,25.407,25.703,647
2026-08-02T22:24:20.4416089Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_no_receivers,4x4/1key_10000msg/u64,21.499,22.641,24.214,647
2026-08-02T22:24:20.4417431Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/arc_sender,4x4/1key_10000msg/u64,38.795,39.590,40.748,647
2026-08-02T22:24:20.4418839Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/clone_sender,4x4/1key_10000msg/u64,45.819,46.920,48.169,647
2026-08-02T22:24:20.4420243Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/guard_held,4x4/1key_10000msg/u64,37.027,38.313,39.696,647
2026-08-02T22:24:20.4421728Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,send_with_receivers/rc_sender,4x4/1key_10000msg/u64,38.481,39.636,40.907,647
2026-08-02T22:24:20.4423185Z panic-fix-clone-sender,9e605bb,x86_64,AMD EPYC 7763 64-Core Processor,local,current_thread_interleaving,1,500,3000,20,flat,phase,subscribe,4x4/1key_10000msg/u64,5715.750,5820.750,6349.500,647
2026-08-02T22:24:20.4454238Z ##[group]Run fail=0