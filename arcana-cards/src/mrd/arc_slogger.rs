//! Arc-Slogger — `{3}{R}{R}` 4/5 red Beast.
//! "{R}, Exile the top ten cards of your library: This creature deals 2 damage to any target."
//!
//! GAP: "Exile the top ten cards of your library" as an activation cost is not
//! expressible in ActivationCost (no exile_top_n field). The mana cost {R} is
//! modeled; the exile-as-cost rider is omitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arc-Slogger");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, Exile the top ten cards of your library: This creature deals 2 damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_two_damage,
            }),
    )
}

fn deal_two_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exile the top ten cards of your library" as activation cost is not
    // modeled (no ActivationCost field for exile-top-N).
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
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 2,
    }]
}
