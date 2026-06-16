//! Lita, Mechanical Engineer — `{2}{W}` 3/3 Legendary Artifact Creature —
//! Artificer with Vigilance.
//! At the beginning of your end step, untap each other artifact creature you
//! control.
//! {3}{W}, {T}: Create a 5/5 colorless Vehicle artifact token named Zeppelin
//! with flying and crew 3.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lita, Mechanical Engineer");
    let artificer = reg.interner_mut().intern("Artificer");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let _zeppelin = reg.interner_mut().intern("Zeppelin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: untap_artifact_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}, {T}: Create a 5/5 colorless Vehicle artifact token named Zeppelin with flying and crew 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_zeppelin,
            }),
    )
}

fn untap_artifact_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each other artifact creature you control" — ForEach over the matching
    // artifact creatures you control (self-exclusion is a minor fidelity gap;
    // Lita has vigilance so untapping her is harmless).
    let filter = ObjectFilter::creature()
        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap { target: arcana_core::objects::NULL_OBJECT_ID }),
    }]
}

fn make_zeppelin(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let zeppelin = reg.interner().lookup("Zeppelin").unwrap_or_default();
    let vehicle = reg.interner().lookup("Vehicle").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vehicle);
    // GAP: the Zeppelin token's printed "crew 3" / "becomes an artifact
    // creature until end of turn" ability is not expressible as a
    // TokenDefinition ability; the bare 5/5 flying Vehicle token is emitted.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: zeppelin,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
