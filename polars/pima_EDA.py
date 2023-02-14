# %% [markdown]
# ## Using `polars`, `pandas`, and `seaborn` to run an EDA on the pima diabetes data set

# %%
import polars as pl
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np
from jmspack.utils import apply_scaling
import session_info
import plotly.express as px
from scipy.stats import pearsonr, beta

# %%
if "jms_style_sheet" in plt.style.available:
    plt.style.use("jms_style_sheet")

# %%
session_info.show(write_req_file=False)

# %%
csv_url = "https://raw.githubusercontent.com/jameshtwose/example_deliverables/main/classification_examples/pima_diabetes/diabetes.csv"
df=pl.read_csv(csv_url)

# %%
df.shape

# %%
df.head()

# %%
len(df.columns)
# np.array(df.columns), np.array(df.dtypes)
df.null_count()

# %%
target="Outcome"

# %%
df.describe()

# %%
df[target].value_counts()

# %%
plot_df = df.to_pandas()
_ = sns.countplot(x=plot_df[target])

# %%
# df.select([pl.col("^run_trial|run$")])
# df.fill_null()

# %%
_ = plt.figure(figsize=(20, 4))
_ = sns.heatmap(plot_df.T)

# %%
px.imshow(plot_df.T.values, 
          x=plot_df.index, 
           y=plot_df.columns, 
           )

# %%
_ = plt.figure(figsize=(20, 4))
_ = sns.heatmap(plot_df
                .pipe(apply_scaling)
                .T)

# %%
_ = plt.figure(figsize=(7, 4))
_ = sns.heatmap(plot_df.corr(), annot=True)

# %%
plot_df.reset_index().melt(id_vars="index")

# %%
# pl.pearson_corr(a=df.select([pl.col("Glucose")]), 
#                 b=df.select([pl.col("BMI")]),
#                 ddof=1)

# %%
r_val, p_val = pearsonr(df.to_pandas()["Glucose"], df.to_pandas()["BMI"])
r_val, p_val
# %%
dist = beta(df.shape[0]/2 - 1,
            df.shape[0]/2 - 1, loc=-1, scale=2)
# %%
p_val_2 = 2*dist.cdf(-abs(r_val))
# %%
np.testing.assert_almost_equal(p_val, p_val_2, decimal=20)

# %%
