//! Light-Paws, Emperor's Voice — `{1}{W}` 2/2 white Legendary Creature — Fox Advisor.
//! "Whenever an Aura you control enters, if you cast it, you may search your
//! library for an Aura card with mana value less than or equal to that Aura
//! and with a different name than each Aura you control, put that card onto
//! the battlefield attached to Light-Paws, then shuffle."
//! The trigger is filtered to Auras you control entering; the search is
//! capped at the entering Aura's mana value (read via `trig.entering_object()`).
//! GAP: "if you cast it" not checkable; "a different name than each Aura you
//! control" not expressible; the found Aura enters unattached rather than
//! attached to Light-Paws; the search is mandatory rather than "may".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Light-Paws, Emperor's Voice");
    let fox = reg.interner_mut().intern("Fox");
    let advisor = reg.interner_mut().intern("Advisor");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::ENCHANTMENT.into())
                        .with_subtype_sym(aura)
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: aura_enters_tutor_aura,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn aura_enters_tutor_aura(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let aura = reg.interner().lookup("Aura").expect("interned at register");
    // "an Aura card with mana value less than or equal to that Aura" — read
    // the entering Aura's mana value from state.
    let Some(mv) = trig.entering_object()
        .and_then(|id| state.objects.get(id))
        .map(|o| o.characteristics.mana_cost.as_ref().map(|c| c.mana_value()).unwrap_or(0))
    else {
        return Vec::new();
    };
    // GAP: "if you cast it" not checkable; "a different name than each Aura
    // you control" not expressible; the found Aura enters unattached rather
    // than attached to Light-Paws; search is mandatory rather than "may".
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent()
            .with_types(TypeLine::ENCHANTMENT.into())
            .with_subtype_sym(aura)
            .with_max_cmc(mv),
        tapped: false,
    }]
}
