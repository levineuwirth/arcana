//! Mnemonic Deluge — `{6}{U}{U}{U}` sorcery. "Exile target instant or
//! sorcery card from a graveyard. Copy that card three times. You may
//! cast the copies without paying their mana costs. Exile Mnemonic
//! Deluge."
//!
//! Only the exile-from-graveyard piece is expressible with the catalog:
//! `Effect::ExileFromGraveyard` on the targeted card. There is no
//! engine primitive for copying a card and casting the copies without
//! paying their mana costs, so that part is GAP'd. Self-exile of the
//! sorcery is not expressible either (no self-exile effect for the
//! resolving spell).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mnemonic Deluge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target instant or sorcery card from a graveyard. Copy that card three times. You may cast the copies without paying their mana costs. Exile Mnemonic Deluge.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    )),
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
    // GAP: copy the exiled card three times and cast the copies without
    // paying their mana costs (no copy-card / cast-copy-free primitive),
    // and self-exile of Mnemonic Deluge (no self-exile effect).
    vec![Effect::ExileFromGraveyard { target: *id }]
}
