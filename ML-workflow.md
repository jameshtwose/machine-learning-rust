## Machine Learning Workflow

**Author:** James Twose <br>
**Date:** 20-03-2021 <br>

---

### Frame the Problem and Look at the Big Picture

1. Define the objective in business terms.
2. How will your solution be used?
3. What are the current solutions/workarounds (if any)?
4. How should you frame this problem (supervised/ unsupervised, online/ offline, regression/ classification etc.)?
5. How should performance be measured?
Is the performance measure aligned with the business objective?
What would be the minimum performance needed to reach the business objective? - cutoffs
6. What are comparable problems? Can you reuse experience or tools?
7. How would you solve the problem manually?
8. List the assumptions you (or others) have made so far.
9. Verify assumptions if possible.

---

### Get the Data

1. List the data you need and how much you need - sample (power) analyses.
2. Document the data you use.
3. Check how much space it will take.
4. Create a workspace (with enough storage space).
5. Get the data.
6. Convert the data to a format you can easily manipulate (without changing the data itself; e.g. .csv, .json).
7. Check the size and type of data (time series, sample, geographical, etc.) and make sure they are what they should be .
8. Sample a test set, put it aside, and never look at it.

**Note:** automate as much as possible so you can easily get fresh data.

--- 

### Perform Exploratory Data Analysis (EDA)

1. Create a copy of the data for exploration (sampling it down to a manageable size if necessary).
2. Create a Jupyter notebook/ python script to keep a record of your data exploration.
3. Study each attribute and its characteristics:
    - Name
    - Type (categorical, int/float, bounded/unbounded, text, structured, etc.) 
    - % of missing values
    - Noisiness and type of noise (stochastic, outliers, rounding errors, etc.) Possibly useful for the task?
    - Type of distribution (Gaussian, Poisson, etc.)
    - Examine outliers and check whether they make sense or not (might require business knowledge)
4. For supervised learning tasks, identify the target attribute(s).
5. Visualize the data.
6. Study the correlations between attributes.
7. Study how you would solve the problem manually.
8. Identify the promising transformations you may want to apply (e.g. log-transformation).
9. Identify extra data that would be useful.
10. Document analysis.

---

### Prepare the Data

- Work on copies of the data (keep the original dataset intact).
- Write functions for all data transformations you apply, for five reasons:
   - So you can easily prepare the data the next time you get a fresh dataset
   - So you can apply these transformations in future projects
   - To clean and prepare the test set
   - To clean and prepare new data instances once your solution is live
   - To make it easy to treat your preparation choices as hyperparameters

1. **Data cleaning:**
   - Fix or remove outliers, if one assess from EDA that outliers are actual abnormal values.
   - Fill in missing values using Imputation techniques (e.g., with zero, mean, median...) or drop their rows (or columns). Note that, packages such as scikit-learn won't allow any model training if the data-frame contain Nan.
2. **Feature selection:**
   - perform feature selection with respect to feature properties via variance analysis as well as percentage of missing values per features
   - perform feature selection with respect to model performance, e.g., lasso regression, random forest
3. **Feature engineering:** (where appropriate)
   - Discretize continuous features (if applicable to analysis).
   - Decompose features (e.g., categorical, date/time, etc.).
   - Add promising transformations of features (e.g., log(x), sqrt(x), x^2, etc.).
   - Aggregate features into promising new features.
4. **Feature scaling:** 
   - standardize or normalize features. Note that, feature scaling is not required for Classification and Regression Tree (CART) model based, e.g., Radom Forest and Boosting algorithms.

---

### Short-List Promising Models

1. Train many quick and dirty models from different categories (e.g., linear, naive Bayes, SVM, Random Forests, neural net, etc.) using standard parameters.
2. Measure and compare their performance.
For each model, use N-fold cross-validation and compute the mean and standard deviation of the performance measure on the N folds.
3. Analyze the most significant variables for each algorithm.
4. Analyze the types of errors the models make.
What data could we use to avoid these errors?
5. Have a quick round of feature selection and engineering.
6. Have one or two more quick iterations of the five previous steps.
7. Short-list the top three to five most promising models, preferring models that make different types of errors.

**Notes:** 

- If the data is huge, you may want to sample smaller training sets so you can train many different models in a reasonable time (be aware that this penalizes complex models such as large neural nets or Random Forests).
- Automate these steps as much as possible.

---

### Fine-Tune the System

1. Fine-tune the hyperparameters using cross-validation.
Treat your data transformation choices as hyperparameters, especially when you are not sure about them (e.g., should I replace missing values with zero or with the median value? Or just drop the rows?).
Unless there are very few hyperparameter values to explore, prefer random search over grid search (otherwise you GridSearchCV). 
2. Try Ensemble methods. Combining your best models will often perform better than running them individually.
3. Once you are confident about your final model, measure its performance on the test set to estimate the generalization error.
Important Note: Don’t tweak your model after measuring the generalization error: you will just start overfitting the test set .

**Notes:**

- You will want to use as much data as possible for this step, especially as you move toward the end of fine-tuning.
- As always automate as much as you can.

---

### Present Your Solution

1. Write out a Method of delivery for the project
2. Create a quick and easy to understand presentation.
Make sure you highlight the big picture first.
3. Explain why your solution achieves the business objective.
4. Don’t forget to present interesting points you noticed along the way.
Describe what worked and what did not.
List your assumptions and your system’s limitations.
5. Ensure your key findings are communicated through beautiful visualizations or easy-to-remember statements (e.g., “the median income is the number-one predictor of housing prices”).
6. If required write a scientific paper.

---

### Model deployment

1. Get your solution ready for production (plug into production data inputs, write unit tests, etc.).
2. Write monitoring code to check your system’s live performance at regular intervals and trigger alerts when it drops (need to be assessed that such system is required, e.g., if we observe data-shifting phenomenon).
Beware of slow degradation too: models tend to “rot” as data evolves.
Monitor your inputs’ quality (e.g., a malfunctioning sensor sending random values, or another team’s output becoming stale). This is particularly important for online learning systems.
3. Retrain your models (if necessary) on a regular basis on fresh data.

**Note:** Remember to automate as much as possible.