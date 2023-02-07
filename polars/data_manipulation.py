# %% [markdown]
# ## Manipulating data with `polars`, `pandas`, and `seaborn`

# %%
import polars as pl
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np
from jmspack.utils import apply_scaling
import session_info

# %%
session_info.show(write_req_file=False)

# %%
# df=pl.read_csv("https://www.stats.govt.nz/assets/Uploads/Annual-enterprise-survey/Annual-enterprise-survey-2021-financial-year-provisional/Download-data/annual-enterprise-survey-2021-financial-year-provisional-csv.csv")

# %%
df=pl.read_csv("https://github.com/mwaskom/Waskom_CerebCortex_2017/blob/master/data/punch_data.csv?raw=true")

# %%
df.shape

# %%
df.head()

# %%
len(df.columns)
# np.array(df.columns), np.array(df.dtypes)
df.null_count()

# %%
# df.select([pl.col("^run_trial|run$")])
# df.fill_null()

# %%
plot_df = (
    df
    .groupby(by=["subj", "run_trial"])
    .mean()
    .to_pandas()
    .drop(["cue_dur", "run"], axis=1)
    .set_index(["subj", "run_trial"])
    .select_dtypes("number")
    .pipe(apply_scaling)
    .sort_index()
)
plot_df.head()

# %%
_ = plt.figure(figsize=(20, 5))
_ = sns.heatmap(data=plot_df.T)


