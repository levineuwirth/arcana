//! Campus Renovation — `{3}{R}{W}` sorcery. "Return up to one target
//! artifact or enchantment card from your graveyard to the battlefield.
//! Exile the top two cards of your library. Until the end of your next
//! turn, you may play those cards."
//!
//! # GAP: "exile top N cards of library and play them until end of next
//! turn" (impulse-draw with a delayed expiry) is not in the Effect catalog.

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
    let name = reg.interner_mut().intern("Campus Renovation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to one target artifact or enchantment card from your graveyard to the battlefield. Exile the top two cards of your library. Until the end of your next turn, you may play those cards.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT)),
                    },
                    count: TargetCount::UpTo(1),
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
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    // GAP: "exile top 2 cards and play until end of next turn" (impulse-draw variant) not in Effect catalog
    effects
}
