//! Shoreline Scout — `{U}` 1/1 Merfolk Scout.
//! "When Shoreline Scout enters the battlefield, you may exile a Merfolk card
//! or a land card from your hand. If you do, conjure a card named Tropical
//! Island into your hand."
//! "As long as another Merfolk or an Island entered the battlefield under your
//! control this turn, Shoreline Scout gets +1/+0." (static — GAP)

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
    let name = reg.interner_mut().intern("Shoreline Scout");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: the "+1/+0 as long as another Merfolk or an Island entered this turn"
    // line is a conditional static self-buff, not a triggered/activated ability,
    // so it is omitted.

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: the ETB payoff is "conjure a card named Tropical Island"
            // (Conjure is an Arena-only mechanic with no Effect::Conjure variant;
            // would need registry-by-name creation in Effect::execute), gated on
            // an optional exile-from-hand additional action; the effect body is
            // left empty.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn noop(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
