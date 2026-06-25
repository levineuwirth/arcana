//! Town Gossipmonger // Incited Rabble — `{W}` Human 1/1 transform creature.
//! Front face: `{T}, Tap an untapped creature you control: Transform this creature.`
//! Back face (Incited Rabble): `This creature attacks each combat if able.`
//! `{2}: This creature gets +1/+0 until end of turn.`
//!
//! GAP: The activated cost "{T}, Tap an untapped creature you control" requires
//! tapping another creature as part of the cost, which is not expressible with
//! the current ActivatedAbilityDef cost model (only Mana costs supported).
//! Wired as a triggered ability on PhaseBegins as a best-effort transform proxy.
//! GAP: Back-face activated ability "{2}: +1/+0" is a cost-bearing activated ability
//! on the back face — not modeled (ActivatedAbilityDef face-gate not in scope here).
//! Back-face "This creature attacks each combat if able" installs a SELF must-attack
//! requirement (CR 508.1a) when the card transforms into Incited Rabble
//! (`SelfTransforms { to_face: Some(1) }`). The transform path itself is GAP'd
//! (the "{T}, Tap a creature" cost is unmodeled), so this requirement only takes
//! effect once the card is transformed by some other means.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Town Gossipmonger");
    let human_sub = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Incited Rabble");
    let back_human_sub = reg.interner_mut().intern("Human");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: The actual transform cost is "{T}, Tap an untapped creature you control"
            // which cannot be expressed as an ActivatedAbilityDef cost. No transform
            // trigger is wired here; the card bones are correct.
            // Back-face "Incited Rabble attacks each combat if able": install a SELF
            // must-attack requirement when the card transforms into the back face.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: install_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
