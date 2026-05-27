//! Yukora, the Prisoner — `{2}{B}{B}` 5/5 Legendary black Demon Spirit.
//! "When Yukora leaves the battlefield, sacrifice all non-Ogre creatures you control."
//! GAP: "leaves the battlefield" trigger — only SelfDies is available; no general
//! "leaves battlefield" (exile, bounce, etc.) trigger.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yukora, the Prisoner");
    let demon = reg.interner_mut().intern("Demon");
    let spirit = reg.interner_mut().intern("Spirit");
    let _ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "leaves the battlefield" — using SelfDies as closest approximation
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: leaves_sacrifice_non_ogres,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn leaves_sacrifice_non_ogres(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ogre_filter = script::subtype_filter(reg, "Ogre")
        .controlled_by(ControllerConstraint::You);
    // Get all creatures you control, exclude Ogres
    let all_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let ogre_ids = script::ids_matching(state, &ogre_filter, trig.controller);
    let non_ogres: Vec<_> = all_creatures
        .into_iter()
        .filter(|id| !ogre_ids.contains(id))
        .collect();
    vec![Effect::ForEach {
        targets: non_ogres,
        effect: Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            count: 1,
        }),
    }]
}
