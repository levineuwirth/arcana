//! Gingerbread Cabin — Land — Forest. "({T}: Add {G}.) This land
//! enters tapped unless you control three or more other Forests. When
//! this land enters untapped, create a Food token."
//!
//! The conditional enters-tapped clause ("unless you control three or
//! more other Forests") is wired via
//! `EntersWithSpec::TappedUnlessControlCount` over a Forest-subtype
//! filter with `3..`. The "enters untapped" gate on the Food trigger is
//! still not expressible; the trigger is wired unconditionally with the
//! gate as a documented gap.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gingerbread Cabin");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(forest);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "Enters tapped unless you control three or more other Forests."
            .with_enters_with(EntersWithSpec::TappedUnlessControlCount {
                filter: ObjectFilter::permanent().with_subtype_sym(forest),
                min: 3,
                max: u32::MAX,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_mana,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Fires only when this land enters UNTAPPED.
                trigger_condition: TriggerCondition::SelfEntersBattlefieldUntapped,
                intervening_if: None,
                effect: etb_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn etb_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}
