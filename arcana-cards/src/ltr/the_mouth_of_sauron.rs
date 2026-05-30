//! The Mouth of Sauron — `{3}{U}{B}` 3/4 Legendary Creature — Human Advisor.
//! "When The Mouth of Sauron enters, target player mills three cards. Then amass
//! Orcs X, where X is the number of instant and sorcery cards in that player's
//! graveyard."
//!
//! GAP: `Effect::Amass` is not modeled in the engine. The mill is expressed;
//! the amass Orcs X step is GAP'd (Vec::new() for that part). We sequence
//! the mill first, then GAP the amass.
//!
//! Keywords: Amass, Mill are rules-text mechanics; no keyword enum entries needed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Mouth of Sauron");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_and_amass,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn etb_mill_and_amass(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // Mill three cards for the target player.
    // GAP: Amass Orcs X (where X = instant and sorcery cards in that player's
    // graveyard) is not expressible — Effect::Amass is not modeled; graveyard
    // card-type counting is not available via script helpers.
    vec![Effect::Mill { player: *p, count: 3 }]
}
