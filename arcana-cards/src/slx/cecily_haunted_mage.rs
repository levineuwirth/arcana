//! Cecily, Haunted Mage — `{1}{U}{B}{R}` 3/5 Legendary Human Wizard.
//! Your maximum hand size is eleven.
//! Whenever Cecily attacks, you draw a card and you lose 1 life. Then if you
//! have eleven or more cards in hand, you may cast an instant or sorcery from
//! your hand without paying its mana cost.
//! Partner—Friends forever.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Cecily, Haunted Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: "Your maximum hand size is eleven." is a static hand-size modifier
    // with no Effect/ability expression.
    // GAP: Partner—Friends forever is a commander-format static; no keyword
    // variant and no rules effect.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_draw_lose,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_draw_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you draw a card and you lose 1 life" expressed; the conditional free-
    // cast rider is GAP'd.
    // GAP: "Then if you have eleven or more cards in hand, you may cast an
    // instant or sorcery from your hand without paying its mana cost." — no
    // free-cast-from-hand Effect is available.
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
