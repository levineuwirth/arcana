//! Ghoulish Procession — `{1}{B}` enchantment.
//! "Whenever one or more nontoken creatures die, create a 2/2 black
//! Zombie creature token with decayed. This ability triggers only
//! once each turn."
//!
//! ZoneChange over nontoken creatures dying with
//! `TriggerFrequency::OncePerTurn` ("only once each turn" — this also
//! approximates the "one or more" batching). GAP: the decayed keyword
//! (can't block; sacrifice at end of attacking combat) is not in the
//! usable keyword surface, so the token is minted without it.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghoulish Procession");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: make_zombie,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 2/2 black Zombie creature token with decayed."
/// GAP: decayed is not an available keyword; token minted without it.
fn make_zombie(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
