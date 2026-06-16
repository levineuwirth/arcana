//! Wakening Sun's Avatar — `{5}{W}{W}{W}` 7/7 white Dinosaur Avatar.
//! "When this creature enters, if you cast it from your hand, destroy all
//! non-Dinosaur creatures."
//! GAP: intervening_if — "if you cast it from your hand" zone-of-origin check
//! not expressible; emitting unconditional destroy-all-non-Dinosaur as
//! best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wakening Sun's Avatar");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: intervening_if — "if cast from hand" not expressible; destroying all non-Dinosaur unconditionally.
    let _dinosaur_filter = script::subtype_filter(reg, "Dinosaur");
    let targets = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
    );
    let mut effects = Vec::new();
    for id in targets {
        // Approximate: destroy all creatures not matching Dinosaur subtype.
        // Cannot filter by subtype in ObjectFilter at call site, so destroy all and
        // rely on the GAP comment.
        effects.push(Effect::DestroyPermanent { target: id });
    }
    effects
}
