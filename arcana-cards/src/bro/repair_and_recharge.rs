//! Repair and Recharge — `{3}{W}{W}` sorcery. "Return target artifact,
//! enchantment, or planeswalker card from your graveyard to the battlefield.
//! Create a tapped Powerstone token."
//
// GAP: "create a tapped Powerstone token" — tapped token creation not
// supported (CreateToken has no `tapped` flag for spell-created tokens).
// Also, TokenDefinition cannot express the activated ability
// "{T}: Add {C}. This mana can't be spent to cast a nonartifact spell."
// ReturnFromGraveyardToBattlefield is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Repair and Recharge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target artifact, enchantment, or planeswalker card from your graveyard to the battlefield. Create a tapped Powerstone token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT | TypeLine::PLANESWALKER)
                        ),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        // GAP: tapped Powerstone token creation (tapped + mana-restriction ability) not expressible
    ]
}
