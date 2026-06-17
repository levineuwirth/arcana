//! Indoctrination Attendant — `{3}{W}` 3/4 white Phyrexian Cleric with
//! Toxic 1. The ETB ability ("you may return another permanent you
//! control to its owner's hand. If you do, create a 1/1 Phyrexian Mite
//! token…") is an optional-permanent-return gating a token creation; the
//! available API has no "you may return a permanent, then if you do …"
//! optional-bounce gate (OptionalPayment only covers mana/life costs),
//! so the conditional follow-up cannot be linked faithfully and the
//! trigger effect is GAP'd. Toxic 1 is emitted as a keyword.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Indoctrination Attendant");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_return_then_mite,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_return_then_mite(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may return another permanent you control to its owner's
    // hand. If you do, create a 1/1 colorless Phyrexian Mite artifact
    // creature token with toxic 1 and 'This token can't block.'" — the
    // optional-bounce-then-conditional-create linkage is not expressible
    // with the demonstrated effect primitives.
    Vec::new()
}
