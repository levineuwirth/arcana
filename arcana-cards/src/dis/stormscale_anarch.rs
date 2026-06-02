//! Stormscale Anarch — `{2}{R}{R}` 2/2 red Lizard Shaman.
//! "{2}{R}, Discard a card at random: This creature deals 2 damage to
//! any target. If the discarded card was multicolored, this creature
//! deals 4 damage instead."
//!
//! Modeled as a mana + discard-a-card activated ability dealing 2
//! damage to any target. The "at random" qualifier on the discard cost
//! and the multicolored-card-checks-for-double conditional are not
//! expressible with the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormscale Anarch");
    let lizard = reg.interner_mut().intern("Lizard");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, Discard a card at random: This creature deals 2 damage to any target. If the discarded card was multicolored, this creature deals 4 damage instead.".into(),
                // GAP: "at random" discard qualifier not modeled — engine
                // discards a chosen card from hand as the cost.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    discard_other: Some(ObjectFilter::default()),
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
    // GAP: "if the discarded card was multicolored, deals 4 instead" —
    // the discarded card's colors aren't inspectable at resolve time;
    // emit the base 2 damage.
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 2,
    }]
}
