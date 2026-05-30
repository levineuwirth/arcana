//! Spellrune Painter // Spellrune Howler — `{2}{R}` Human Shaman Werewolf 2/3 (front) /
//! Werewolf (back).
//!
//! Front face:
//!   Whenever you cast an instant or sorcery spell, this creature gets +1/+1 until end of turn.
//!   Daybound (not modeled — see GAP).
//!
//! Back face (Spellrune Howler):
//!   Whenever you cast an instant or sorcery spell, this creature gets +2/+2 until end of turn.
//!   Nightbound (not modeled — see GAP).
//!
//! GAP: Daybound/Nightbound keywords not in the supported keyword set — day/night cycle is
//!      engine debt; not emitted.
//! GAP: back-face-only triggered ability (+2/+2 pump) not auto-installed on transform.
//! GAP: werewolf day/night transform triggers not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::layers::Duration;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spellrune Painter");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Daybound not in supported keyword set
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Spellrune Howler — Werewolf
    let back_name = reg.interner_mut().intern("Spellrune Howler");
    let werewolf_back = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            // GAP: Nightbound not in supported keyword set
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: whenever you cast an instant or sorcery, +1/+1 until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::INSTANT | TypeLine::SORCERY),
                        ),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face-only triggered ability (+2/+2 on instant/sorcery cast) not modeled.
            // GAP: day/night transform triggers not modeled.
    )
}

fn front_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
