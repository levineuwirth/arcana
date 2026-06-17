//! Trostani, Selesnya's Voice — `{G}{G}{W}{W}` 2/5 Legendary Dryad.
//! "Whenever another creature you control enters, you gain life equal to that
//! creature's toughness."
//! "{1}{G}{W}, {T}: Populate." (the Populate effect is GAP'd — no Effect
//! variant.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trostani, Selesnya's Voice");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_life_equal_toughness,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}{W}, {T}: Populate.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: populate,
            }),
    )
}

fn gain_life_equal_toughness(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.entering_object().unwrap_or(trig.source);
    let n = script::toughness_of(state, id).max(0) as u32;
    vec![Effect::GainLife { player: trig.controller, amount: n }]
}

fn populate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Populate (create a token that's a copy of a creature token you
    // control) has no Effect variant in the demonstrated API.
    Vec::new()
}
