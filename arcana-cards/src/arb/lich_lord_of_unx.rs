//! Lich Lord of Unx — `{1}{U}{B}` 2/2 Zombie Wizard.
//! "{U}{B}, {T}: Create a 1/1 blue and black Zombie Wizard creature token." /
//! "{U}{U}{B}{B}: Target player loses X life and mills X cards, where X is the
//! number of Zombies you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lich Lord of Unx");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    // GAP: "Mill" keyword has no KeywordAbility variant; keywords empty.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{B}, {T}: Create a 1/1 blue and black Zombie Wizard creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_zombie,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{U}{B}{B}: Target player loses X life and mills X cards, where X is the number of Zombies you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{U}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: drain_and_mill,
            }),
    )
}

fn make_zombie(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let wizard = reg.interner().lookup("Wizard").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn drain_and_mill(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Zombie").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![
        Effect::LoseLife { player: *p, amount: x },
        Effect::Mill { player: *p, count: x },
    ]
}
