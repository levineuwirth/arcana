//! Caribou Range — `{2}{W}{W}` enchantment — Aura.
//! "Enchant land you control.
//!  Enchanted land has '{W}{W}, {T}: Create a 0/1 white Caribou creature token.'
//!  Sacrifice a Caribou token: You gain 1 life."
//!
//! Host-activated grant: an ETB-installed `attached_activated` continuous
//! effect grants the enchanted land a `{W}{W}, {T}` ability creating a 0/1
//! white Caribou. The Aura's own "Sacrifice a Caribou token: gain 1 life"
//! ability is not expressible (sacrifice-a-token cost), so it is omitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caribou Range");
    let aura = reg.interner_mut().intern("Aura");
    let _caribou = reg.interner_mut().intern("Caribou");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: controller wording approximated by caster's choice
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let ability = ActivatedAbilityDef {
        text: "{W}{W}, {T}: Create a 0/1 white Caribou creature token.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{W}{W}").expect("valid cost"),
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: make_caribou,
    };
    // GAP: Aura's own "Sacrifice a Caribou token: gain 1 life" (sacrifice-a-token cost)
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            trig.source,
            ability,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn make_caribou(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let caribou = reg
        .interner()
        .lookup("Caribou")
        .expect("Caribou interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(caribou);
    let token = TokenDefinition {
        name: caribou,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
