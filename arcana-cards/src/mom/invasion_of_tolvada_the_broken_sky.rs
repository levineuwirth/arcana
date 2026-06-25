//! Invasion of Tolvada // The Broken Sky — `{3}{W}{B}` Battle — Siege.
//! Front (Siege, 4 defense counters): When this Siege enters, return target
//! nonbattle permanent card from your graveyard to the battlefield.
//! Back (Enchantment): Creature tokens you control get +1/+0 and have
//! lifelink. At the beginning of your end step, create a 1/1 white and
//! black Spirit creature token with flying.
//!
//! Back face (The Broken Sky — Enchantment):
//! - Static "Creature tokens you control get +1/+0 and have lifelink" — installed as a
//!   filtered_pump + filtered_keyword (tokens_only creatures you control) on the
//!   SelfTransforms{to_face:1} trigger with Duration::WhileSourceShowsFace(1) (dormant on
//!   the battle face, live on the back face).
//! - "At the beginning of your end step, create a 1/1 W/B Spirit with flying" — face-gated
//!   to the back face (face 1).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Tolvada");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    // Pre-intern Spirit for use in the token-creation trigger
    let _ = reg.interner_mut().intern("Spirit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: The Broken Sky — Enchantment (W/B)
    let back_name = reg.interner_mut().intern("The Broken Sky");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            // ETB trigger: return target nonbattle permanent card from graveyard
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().without_types(TypeLine::BATTLE.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
            })
            // Back-face ability: at the beginning of your end step, create a Spirit token.
            // Face-gated to the back face (face 1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_spirit_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1)
            // Back-face static: "Creature tokens you control get +1/+0 and have lifelink."
            // Installed on the defeat→back-face transform, live only while showing face 1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: install_token_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_token_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .tokens_only();
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                0,
                Duration::WhileSourceShowsFace(1),
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Lifelink,
                Duration::WhileSourceShowsFace(1),
            ),
        },
    ]
}

fn etb_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

fn create_spirit_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit_sym = reg
        .interner()
        .lookup("Spirit")
        .expect("Spirit interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spirit_sym);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit_sym,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
