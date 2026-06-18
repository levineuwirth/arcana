//! Survey Mechan — `{4}` 1/3 Artifact Creature — Robot with Flying and Hexproof.
//! "{10}, Sacrifice this creature: It deals 3 damage to any target. Target player
//! draws three cards and gains 3 life. This ability costs {X} less to activate,
//! where X is the number of differently named lands you control."
//!
//! GAP: the dynamic cost reduction ("{X} less, X = differently named lands you
//! control") is not expressible — ActivationCost mana is a fixed ManaCost with no
//! dynamic-reduction field; the printed {10} is charged in full.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Survey Mechan");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{10}, Sacrifice this creature: It deals 3 damage to any target. Target player draws three cards and gains 3 life.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{10}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![
                TargetRequirement::any_target(),
                TargetRequirement::target_player(),
            ],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: damage_and_draw,
        }),
    )
}

fn damage_and_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(t) = ctx.targets.targets.first() {
        let dt = match t {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        effects.push(Effect::DealDamage { source: ctx.source, target: dt, amount: 3 });
    }
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.get(1) {
        effects.push(Effect::DrawCards { player: *p, count: 3 });
        effects.push(Effect::GainLife { player: *p, amount: 3 });
    }
    effects
}
