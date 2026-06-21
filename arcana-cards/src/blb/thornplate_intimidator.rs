//! Thornplate Intimidator — `{3}{B}` 4/3 Rat Rogue.
//! "Offspring {3}. When this creature enters, target opponent loses 3
//!  life unless they sacrifice a nonland permanent of their choice or
//!  discard a card."
//!
//! Offspring is not in the usable keyword surface and the offspring
//! token-copy rider is GAP'd. The ETB punisher's "unless they
//! sacrifice ... or discard a card" is a player-chosen alternative
//! payment; `OptionalPaymentKind` only offers Mana / Life, so neither
//! the sacrifice nor the discard branch is expressible as a payment
//! gate. The whole conditional drain is GAP'd rather than emit a
//! materially-wrong unconditional drain. The targeting (target
//! opponent) is preserved so the trigger shape is faithful.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thornplate Intimidator");
    let rat = reg.interner_mut().intern("Rat");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword "Offspring {3}" — not in the usable keyword surface; the additional-cost token-copy rider is unexpressible.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_drain_unless,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn etb_drain_unless(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(_p) = target else { return Vec::new(); };
    // GAP: "loses 3 life unless they sacrifice a nonland permanent of their choice or discard a card" — player-chosen alternative payment (sacrifice OR discard); OptionalPaymentKind offers only Mana/Life, so neither branch is expressible. Whole conditional drain omitted to avoid an unconditional (wrong) drain.
    Vec::new()
}
