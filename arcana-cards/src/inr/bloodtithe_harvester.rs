//! Bloodtithe Harvester — `{B}{R}` 3/2 black/red Vampire.
//!
//! * "When this creature enters, create a Blood token." — a
//!   `SelfEntersBattlefield` trigger minting one Blood commodity token.
//! * "{T}, Sacrifice this creature: Target creature gets -X/-X until end
//!   of turn, where X is twice the number of Blood tokens you control."
//!   — a tap + sacrifice-self activation; X is computed at resolution as
//!   twice the count of Blood tokens you control, applied via a negative
//!   `Effect::Pump`. ("Activate only as a sorcery" → `is_instant_speed:
//!   false`, the default for permanent activated abilities.)

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodtithe Harvester");
    let vampire = reg.interner_mut().intern("Vampire");
    let _blood = reg.interner_mut().intern("Blood");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_blood,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice Bloodtithe Harvester: Target creature gets -X/-X until end of turn, where X is twice the number of Blood tokens you control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_by_blood,
            }),
    )
}

/// ETB: create a Blood token.
fn etb_make_blood(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Blood,
        count: 1,
    }]
}

/// "Target creature gets -X/-X, where X is twice the number of Blood tokens
/// you control."
fn minus_x_by_blood(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let blood = script::subtype_filter(reg, "Blood").controlled_by(ControllerConstraint::You);
    let x = (script::count_matching(state, &blood, ctx.controller) * 2) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
