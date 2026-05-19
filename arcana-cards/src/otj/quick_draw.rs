//! Quick Draw — `{R}` instant. "Target creature you control gets +1/+1 and gains
//! first strike until end of turn. Creatures target opponent controls lose first
//! strike and double strike until end of turn."
//!
//! GAP: stripping keywords from a set of creatures (all creatures an opponent
//! controls lose first strike and double strike until end of turn) is not
//! expressible with the catalog's Effect variants (no Effect::RemoveKeyword or
//! ForEach variant for keyword removal).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quick Draw");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+1 and gains first strike until end of turn. Creatures target opponent controls lose first strike and double strike until end of turn.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Player,
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
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = t0 else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::FirstStrike],
        },
        // GAP: all creatures an opponent controls lose FirstStrike and DoubleStrike
        // until end of turn — no Effect::RemoveKeyword or equivalent in catalog
    ]
}
