//! Jorn, God of Winter // Kaldring, the Rimestaff
//! Front: {2}{G} Legendary Snow Creature — God 3/3 (green)
//! Whenever Jorn attacks, untap each snow permanent you control.
//! Back: Legendary Snow Artifact
//! {T}: You may play target snow permanent card from your graveyard this turn.
//!      If you do, it enters tapped.
//! GAP: "snow permanent" filter not expressible (no snow supertype filter in ObjectFilter);
//!      Jorn's attack trigger untaps ALL permanents you control (over-broad approximation).
//! GAP: Back face activated ability ({T}: play snow permanent from graveyard, enters tapped)
//!      not modeled — back-face-only activated abilities and "play from graveyard tapped" are
//!      engine debt.
//! GAP: Snow supertype not modeled on characteristics.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jorn, God of Winter");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Kaldring, the Rimestaff");

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: jorn_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn jorn_attacks(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: should target only snow permanents; untapping all permanents you control is over-broad.
    let filter = ObjectFilter::permanent().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
