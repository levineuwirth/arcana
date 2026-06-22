//! Spider-Slayer, Hatred Honed — `{2}` 2/1 Legendary Artifact Creature —
//! Human Villain (colorless).
//! "Whenever Spider-Slayer deals damage to a Spider, destroy that creature.
//!  {6}, Exile this card from your graveyard: Create two tapped 1/1
//!  colorless Robot artifact creature tokens with flying."
//!
//! Both abilities are expressed.
//! GAP: the Robot tokens are printed "tapped"; there is no plain
//!      create-tapped-token Effect (only CreateToken / TappedAttacking), so
//!      they are minted untapped (best-effort; the rest of the token is
//!      faithful).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spider-Slayer, Hatred Honed");
    let human = reg.interner_mut().intern("Human");
    let villain = reg.interner_mut().intern("Villain");
    let _robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(villain);
    let spider_filter = script::subtype_filter(reg, "Spider");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Permanent(spider_filter),
                    combat_only: false,
                },
                intervening_if: None,
                effect: destroy_damaged_spider,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}, Exile this card from your graveyard: Create two \
                       tapped 1/1 colorless Robot artifact creature tokens \
                       with flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: make_robots,
            }),
    )
}

fn destroy_damaged_spider(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::DestroyPermanent { target: id }]
}

fn make_robots(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let robot = reg.interner().lookup("Robot").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let token = TokenDefinition {
        name: robot,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}
