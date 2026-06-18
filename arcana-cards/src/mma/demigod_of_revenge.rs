//! Demigod of Revenge — `{B/R}{B/R}{B/R}{B/R}{B/R}` 5/4 Spirit Avatar
//! with Flying and Haste.
//! "When you cast this spell, return all cards named Demigod of Revenge
//! from your graveyard to the battlefield."
//!
//! Flying + Haste are base keywords. The cast trigger is wired as a
//! SpellCast (filtered to this card's name, caster = You) firing from the
//! Stack zone. The effect — "return ALL cards named ~ from your graveyard
//! to the battlefield" — has no expressible primitive (no graveyard-all-
//! return-by-name effect; Reanimate is a single non-named pick), so the
//! body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demigod of Revenge");
    let spirit = reg.interner_mut().intern("Spirit");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(avatar);

    let self_name = reg.interner().lookup("Demigod of Revenge");

    let chars = Characteristics {
        name,
        mana_cost: Some(
            ManaCost::parse("{B/R}{B/R}{B/R}{B/R}{B/R}").expect("valid cost"),
        ),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter {
                    name: self_name,
                    ..ObjectFilter::default()
                }),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cast_return_all,
            trigger_zones: vec![Zone::Stack],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_return_all(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return all cards named Demigod of Revenge from your graveyard
    // to the battlefield" — no graveyard-all-return-by-name primitive.
    Vec::new()
}
