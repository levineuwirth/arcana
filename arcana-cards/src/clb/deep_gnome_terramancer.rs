//! Deep Gnome Terramancer — `{1}{W}` 2/2 Creature — Gnome Wizard. Mono-white.
//!
//! Oracle:
//! - Flash — keyword. ("Mold Earth" is an ability word labelling the trigger,
//!   not a usable KeywordAbility.)
//! - "Mold Earth — Whenever one or more lands enter under an opponent's control
//!   without being played, you may search your library for a Plains card, put
//!   it onto the battlefield tapped, then shuffle. Do this only once each
//!   turn." — ZoneChange trigger (a land an opponent controls → battlefield),
//!   `TutorToBattlefield` for a Plains tapped, `OncePerTurn`.
//!   GAP: the "without being played" qualifier is not expressible (the trigger
//!   fires on any opponent land entering).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deep Gnome Terramancer");
    let gnome = reg.interner_mut().intern("Gnome");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: tutor_plains_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn tutor_plains_tapped(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Plains")
        .with_types(TypeLine::LAND.into());
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: true,
    }]
}
