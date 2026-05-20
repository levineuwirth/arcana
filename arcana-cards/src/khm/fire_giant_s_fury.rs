//! Fire Giant's Fury — `{1}{R}` sorcery. "Target Giant you control
//! gets +2/+2 and gains trample until end of turn. Whenever it deals
//! combat damage to a player this turn, exile that many cards from the
//! top of your library. Until the end of your next turn, you may play
//! those cards."
//!
//! Only the +2/+2 + trample pump is expressible; the delayed
//! combat-damage exile-and-play rider is not.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire Giant's Fury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target Giant you control gets +2/+2 and gains trample \
                   until end of turn. Whenever it deals combat damage to a \
                   player this turn, exile that many cards from the top of \
                   your library. Until the end of your next turn, you may \
                   play those cards."
                .into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: delayed "whenever it deals combat damage to a player, exile
    // that many cards and you may play them" rider is not expressible.
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}
