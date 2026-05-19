//! Grave Exchange — `{4}{B}{B}` sorcery. "Return target creature card from
//! your graveyard to your hand. Target player sacrifices a creature of their
//! choice."
//!
//! # GAP: SacrificeCreature — no Effect variant for 'target player sacrifices
//!   a creature of their choice'

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
    let name = reg.interner_mut().intern("Grave Exchange");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to your hand. Target player sacrifices a creature of their choice.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_player(),
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
    // GAP: SacrificeCreature — no Effect variant for 'target player sacrifices a creature'
    let mut targets = entry.targets.targets.iter();
    let Some(t1) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(card_id) = t1 else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *card_id }]
}
