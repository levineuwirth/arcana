//! Gnomeball Machine — artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, create two 1/1 colorless Gnome
//! artifact creature tokens."
//!
//! Contraption cranking (an Un-set subsystem) has no TriggerCondition;
//! the nearest self-event (`SelfBecomesTapped`) is wired as a
//! placeholder with a GAP note. The token mint itself is faithful.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnomeball Machine");
    let contraption = reg.interner_mut().intern("Contraption");
    let _gnome = reg.interner_mut().intern("Gnome");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever you crank this Contraption"
                // (Un-set cranking) has no TriggerCondition variant;
                // SelfBecomesTapped is the nearest self-event placeholder.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: mint_gnomes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create two 1/1 colorless Gnome artifact creature tokens."
fn mint_gnomes(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let gnome = reg.interner().lookup("Gnome").unwrap_or_default();
    let gnome_token = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(gnome);
        TokenDefinition {
            name: gnome,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: gnome_token(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: gnome_token(),
        },
    ]
}
