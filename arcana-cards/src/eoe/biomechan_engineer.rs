//! Biomechan Engineer — `{G}{U}` 2/2 Insect Artificer.
//! "When this creature enters, create a Lander token." (An artifact with
//! "{2}, {T}, Sacrifice this token: Search your library for a basic land
//! card, put it onto the battlefield tapped, then shuffle.")
//! "{8}: Draw two cards and create a 2/2 colorless Robot artifact
//! creature token."
//!
//! The ETB mints a bare Lander artifact token; its activated sacrifice
//! ability can't be attached to a token (TokenDefinition carries only
//! triggered abilities), so that ability is a GAP. The {8} activation
//! draws two and mints the Robot token.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biomechan Engineer");
    let insect = reg.interner_mut().intern("Insect");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern token subtype names so the resolvers can look them up.
    let _lander = reg.interner_mut().intern("Lander");
    let _robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: the Lander token's "{2},{T},Sacrifice: search for a basic
            // land" activated ability can't be attached to a TokenDefinition.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_lander,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{8}: Draw two cards and create a 2/2 colorless Robot artifact creature token."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{8}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_and_make_robot,
            }),
    )
}

fn etb_create_lander(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let lander = reg.interner().lookup("Lander").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: lander,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn draw_and_make_robot(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let robot = reg.interner().lookup("Robot").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: robot,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
