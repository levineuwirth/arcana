//! Rin and Seri, Inseparable — `{1}{R}{G}{W}` 4/4 Legendary Creature — Dog
//! Cat.
//! "Whenever you cast a Dog spell, create a 1/1 green Cat creature token."
//! "Whenever you cast a Cat spell, create a 1/1 white Dog creature token."
//! "{R}{G}{W}, {T}: Rin and Seri deals damage to any target equal to the
//! number of Dogs you control. You gain life equal to the number of Cats you
//! control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rin and Seri, Inseparable");
    let dog = reg.interner_mut().intern("Dog");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(cat);

    let dog_filter = script::subtype_filter(reg, "Dog");
    let cat_filter = script::subtype_filter(reg, "Cat");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(dog_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_cat_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(cat_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_dog_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{G}{W}, {T}: Rin and Seri deals damage to any target equal to the number of Dogs you control. You gain life equal to the number of Cats you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{G}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_and_gain,
            }),
    )
}

fn make_cat_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: cat,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn make_dog_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dog = reg.interner().lookup("Dog").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dog,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn damage_and_gain(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dogs = script::count_matching(
        state,
        &script::subtype_filter(reg, "Dog").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let cats = script::count_matching(
        state,
        &script::subtype_filter(reg, "Cat").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage {
            source: ctx.source,
            target: dt,
            amount: dogs,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: cats,
        },
    ]
}
