//! Harold and Bob, First Numens — `{2}{G}` 3/3 Legendary Treefolk Mutant with
//! Vigilance and Reach.
//!
//! Oracle:
//! * Vigilance, reach (keywords).
//! * When Harold and Bob dies, if it was a creature, return it to the
//!   battlefield. It's an Aura enchantment with enchant Forest you control and
//!   "Enchanted Forest has '{T}: Add three mana of any one color. You get two
//!   rad counters.'" Harold and Bob loses all other abilities.
//!
//! The dies trigger's effect is fully GAP'd: it returns the creature as an Aura
//! enchantment with a runtime-built enchant restriction and a granted mana
//! ability, plus "loses all other abilities" — none of these (self-reanimation
//! as an Aura, dynamic enchant target, granted activated mana ability with rad
//! counters) are expressible with the available primitives. The intervening
//! "if it was a creature" check is likewise GAP'd (no matching condition
//! helper), so the trigger is emitted with no effect for catalog presence.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harold and Bob, First Numens");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            // GAP: intervening "if it was a creature" — no matching condition helper.
            intervening_if: None,
            effect: dies_become_aura,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_become_aura(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return it to the battlefield as an Aura enchantment with enchant
    // Forest you control and a granted mana ability, losing all other
    // abilities" — self-reanimation as an Aura with a dynamic enchant target
    // and a runtime-granted activated mana ability is not expressible.
    Vec::new()
}
