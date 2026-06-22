//! Wildfire Awakener — `{X}{1}{R}{W}` 3/2 red-white Human Wizard with Convoke.
//!
//! "Convoke
//!  When this creature enters, create X 1/1 red Elemental creature tokens with
//!  'Whenever this token becomes tapped, it deals 1 damage to target player.'"
//!
//! Convoke is a cost-reduction/payment keyword with no expressible
//! `KeywordAbility` variant — GAP.
//!
//! The ETB trigger creates a NUMBER of tokens equal to the X paid for the
//! spell. A triggered-ability effect fn has no accessor for the spell's chosen
//! X value (PendingTrigger exposes no x_value), and there is no `script::`
//! helper that recovers it, so the dynamic count is uncomputable. Per the
//! dynamic-amount rule (never hardcode a literal where the text scales) the
//! whole ETB effect is GAP'd.

// GAP (keyword): Convoke — alternative-cost payment not expressible.
// GAP (trigger): "create X 1/1 red Elemental tokens with a becomes-tapped
// ability" — X comes from the spell's chosen X value, which a trigger effect fn
// cannot read (no x_value accessor on PendingTrigger / no script:: helper).

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
    let name = reg.interner_mut().intern("Wildfire Awakener");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_x_elementals,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_x_elementals(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: count is the spell's X (uncomputable in a trigger effect fn).
    Vec::new()
}
