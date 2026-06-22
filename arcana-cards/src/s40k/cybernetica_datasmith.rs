//! Cybernetica Datasmith — `{1}{U}{B}` 0/1 Artifact Creature — Human Artificer.
//! Protection from Robots.
//! Field Reprogramming — {U}, {T}: Target player draws a card. Another target
//! player creates a 4/4 colorless Robot artifact creature token with "This token
//! can't block."
//!
//! The activated ability is wired with two player targets (first draws, second
//! mints the Robot). "Protection from Robots" is not an available KeywordAbility
//! (GAP), and the Robot token's "can't block" static is a GAP on the token body.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cybernetica Datasmith");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let _robot = reg.interner_mut().intern("Robot");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP keyword: "Protection from Robots" — Protection not available.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Target player draws a card. Another target player \
                       creates a 4/4 colorless Robot artifact creature token with \
                       \"This token can't block.\""
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement::target_player(),
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: field_reprogramming,
            }),
    )
}

fn field_reprogramming(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() {
        effects.push(Effect::DrawCards { player: *p, count: 1 });
    }
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.get(1) {
        let robot = reg.interner().lookup("Robot").unwrap_or_default();
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(robot);
        // GAP: Robot token's "This token can't block." static is unexpressible.
        effects.push(Effect::CreateToken {
            controller: *p,
            token: TokenDefinition {
                name: robot,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(4)),
                toughness: Some(PtValue::Fixed(4)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
