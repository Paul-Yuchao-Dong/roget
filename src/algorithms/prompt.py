
# %%
import sys

h = [None]

class HistoryPrompt:
    "Create a prompt that stores results (i.e. _) in the list h."
    def __init__(self, str='h[%d] >>> '):
        self.str = str;
        
    def __str__(self):
        try:
            if _ not in [h[-1], None, h]: h.append(_);
        except NameError: # _ not defined yet
            pass 
        return self.str % len(h);
    
    def __radd__(self, other):
        return str(other) + str(self)

sys.ps1 = HistoryPrompt()