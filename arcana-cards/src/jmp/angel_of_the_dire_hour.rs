//! Angel of the Dire Hour — `{5}{W}{W}` 5/4 Angel with Flash and Flying.
//!
//! * Flash, Flying (keywords).
//! * "When this creature enters, if you cast it from your hand, exile all
//!   attacking creatures." — the ETB trigger is wired, but GAP on two
//!   counts: (1) "if you cast it from your hand" has no `conditions::`
//!   intervening-if predicate, and (2) there is no attacking-creature
//!   filter in the demonstrated `ObjectFilter` refinements to enumerate
//!   "all attacking creatures". The effect body is therefore empty.

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
    let name = reg.interner_mut().intern("Angel of the Dire Hour");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "if you cast it from your hand" has no conditions:: predicate, so
    // intervening_if stays None.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_all_attackers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_all_attackers(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no attacking-creature ObjectFilter refinement to enumerate "all
    // attacking creatures" for a board-wide exile.
    Vec::new()
}
