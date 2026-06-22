//! Aerie Ouphes — `{4}{G}` 3/3 green Ouphe with Persist.
//!
//! * "Sacrifice this creature: It deals damage equal to its power to target
//!   creature with flying." — a sacrifice-self activated ability; the damage
//!   amount is the source's power at resolution (`script::power_of`), and the
//!   target is a creature with flying.
//! * Persist — keyword line.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aerie Ouphes");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Persist],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice this creature: It deals damage equal to its power to target creature with flying.".into(),
            cost: ActivationCost {
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: deal_power_to_flyer,
        }),
    )
}

fn deal_power_to_flyer(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let amount = script::power_of(state, ctx.source).max(0) as u32;
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
