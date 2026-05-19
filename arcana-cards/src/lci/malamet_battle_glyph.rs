//! Malamet Battle Glyph — `{G}` sorcery.
//! "Choose target creature you control and target creature you don't
//! control. If the creature you control entered this turn, put a +1/+1
//! counter on it. Then those creatures fight each other."
//
// GAP: conditional "if the creature you control entered this turn" for
//      the counter — no Effect::Conditional with ETB-this-turn check.
//      Best effort: fight only (counter omitted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malamet Battle Glyph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature you don't control. If the creature you control entered this turn, put a +1/+1 counter on it. Then those creatures fight each other.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut iter = entry.targets.targets.iter();
    let first = iter.next();
    let second = iter.next();
    let (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) = (first, second) else {
        return Vec::new();
    };
    // GAP: conditional +1/+1 counter if controlled creature entered this turn
    vec![Effect::Fight { a: *a, b: *b }]
}
