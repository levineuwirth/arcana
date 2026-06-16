//! Maximus, Knight Apparent — `{3}{R}` 4/4 Legendary Human Knight with Trample.
//! ETB: you may search your library for a mana-value-2 Equipment card, reveal,
//!   put into hand, then shuffle.
//! {1}, Sacrifice an artifact: You get {E}{E}.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maximus, Knight Apparent");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    // Pre-intern the Equipment subtype so the resolver's subtype_filter lookup
    // succeeds (subtype_filter matches nothing on a never-interned name).
    let _equipment = reg.interner_mut().intern("Equipment");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice an artifact: You get {E}{E}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: get_two_energy,
            }),
    )
}

fn etb_tutor_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may search ... for an Equipment card with mana value 2". The "may"
    // is a resolution-time choice; TutorToHand posts the optional search.
    let filter = script::subtype_filter(reg, "Equipment").with_max_cmc(2);
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}

fn get_two_energy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainEnergy {
        player: ctx.controller,
        amount: 2,
    }]
}
