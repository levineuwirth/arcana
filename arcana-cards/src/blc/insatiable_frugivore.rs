//! Insatiable Frugivore — `{3}{B}` 2/4 black Rat Berserker.
//!
//! When this creature enters, create a Food token, then you may exile
//! three cards from your graveyard. If you do, repeat this process.
//! {3}{B}, Sacrifice X Foods: Creatures you control get +X/+0 and gain
//! menace until end of turn. — GAP (variable-X sacrifice cost).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "{3}{B}, Sacrifice X Foods: Creatures you control get +X/+0 and gain
// menace until end of turn." The cost is sacrificing a VARIABLE number X of
// Foods; ActivationCost has no variable-X sacrifice cost, and the payload then
// scales by that X — neither the cost nor the X-scaled pump is expressible.
// (`sacrifice_other` only sacrifices a single chosen permanent.)

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Insatiable Frugivore");
    let rat = reg.interner_mut().intern("Rat");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_food,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: create one Food token. The "then you may exile three cards from your
/// graveyard; if you do, repeat this process" recursion is GAP'd — the only
/// OptionalPayment cost kinds are Mana/Life (no exile-from-graveyard cost), so
/// the repeat loop can't be gated; the guaranteed first Food is still minted.
fn etb_food(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "then you may exile three cards from your graveyard. If you do,
    // repeat this process." — no exile-from-graveyard OptionalPayment cost.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}
