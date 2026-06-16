//! Pack Guardian — `{2}{G}{G}` 4/3 green Wolf Spirit with Flash.
//! "When this creature enters, you may discard a land card. If you do,
//! create a 2/2 green Wolf creature token."
//!
//! Flash is a base keyword. The ETB trigger is wired, but its "you may
//! discard a land card. If you do, …" gate is unexpressible: the only
//! `OptionalPaymentKind` variants are `Mana`/`Life` — there is no
//! discard-a-card optional cost — so the conditional discard → token
//! clause is GAP'd rather than firing the token unconditionally.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Pack Guardian");
    let wolf = reg.interner_mut().intern("Wolf");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_discard_land_for_wolf,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_discard_land_for_wolf(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a land card. If you do, create a 2/2 green Wolf"
    // — OptionalPaymentKind has only Mana/Life, so a "discard a land card"
    // optional cost gate is not expressible; emitting the token would be a
    // wrong (unconditional) card, so the whole conditional clause is GAP'd.
    Vec::new()
}
