//! Mold Shambler — `{3}{G}` 3/3 Fungus Beast.
//!
//! * Kicker {1}{G}.
//! * When this creature enters, if it was kicked, destroy target noncreature
//!   permanent.
//!
//! Kicker is not in the usable KeywordAbility surface and there is no "was
//! kicked" intervening-if condition, so the kicker cost is GAP'd (no keyword
//! emitted) and the kicker-conditional ETB destroy is GAP'd: firing it
//! unconditionally would be a materially wrong card (it would destroy even
//! when the spell was cast without paying the kicker).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mold Shambler");
    let fungus = reg.interner_mut().intern("Fungus");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Kicker {1}{G} — Kicker is not in the usable KeywordAbility surface.
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if it was kicked" — no "was kicked" intervening-if
                //       condition exists; gating on kicker is unexpressible.
                intervening_if: None,
                effect: etb_if_kicked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_if_kicked(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was kicked, destroy target noncreature permanent" — the
    //       kicker gate has no condition; firing the destroy unconditionally
    //       would be wrong, so emit nothing.
    Vec::new()
}
