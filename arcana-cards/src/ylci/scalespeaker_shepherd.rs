//! Scalespeaker Shepherd — `{2}{G}` 2/1 Human Druid.
//! "When Scalespeaker Shepherd enters, draft a card from Scalespeaker Shepherd's
//! spellbook."
//! "Dinosaur spells you cast cost {1} less to cast."

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
    let name = reg.interner_mut().intern("Scalespeaker Shepherd");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draft_spellbook,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: static "Dinosaur spells you cast cost {1} less" is a cost-reduction
    // continuous static ability, not a triggered/activated ability — not
    // expressible with the demonstrated MultiAbilityCreature API.
}

fn etb_draft_spellbook(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draft a card from ~'s spellbook" is an Alchemy/spellbook mechanic
    // (Conjure-adjacent, registry-by-name spellbook creation) with no Effect
    // variant in the demonstrated API.
    Vec::new()
}
